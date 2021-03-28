//! Difficulty calculation

use std::cmp::Reverse;
use std::mem;
use std::ops::{Index, IndexMut};

use ordered_float::NotNan;

use crate::beatmap::Beatmap;
use crate::enums::{Mode, Mods};
use crate::hitobject::{HitObject, HitObjectKind};
use crate::math::Point;

use super::pp_calc::{mods_apply, ModsApply};

/// Difficulty calculator
#[derive(Clone)]
pub struct DiffCalc<'a> {
    /// Beatmap reference
    beatmap: &'a Beatmap,

    /// Hit objects
    hit_objects: Vec<CalcHitObject<'a>>,

    /// Strains
    strains: Vec<f64>,
}

/// Wrapper around hit object used for difficulty calculation
#[derive(Clone)]
pub struct CalcHitObject<'a> {
    /// Hit object reference
    inner: &'a HitObject,

    /// Strains
    strains: ObjectStrains,

    /// Is single tap
    is_single: bool,

    /// normpos
    normpos: Point<f64>,

    /// TODO: what is this???
    d_distance: f64,

    /// time since previous object? probably
    delta_time: f64,

    /// angle from previous object? probably
    angle: Option<f64>,
}

impl<'a> CalcHitObject<'a> {
    #[inline]
    /// Convenience function for getting start time as an i32
    pub fn time(&self) -> i32 {
        self.inner.start_time.0
    }

    #[inline]
    /// Convenience function for getting start time as an f64
    pub fn timef(&self) -> f64 {
        self.time() as f64
    }
}

/// Type of difficulty used for calculation
#[derive(Copy, Clone, Debug)]
pub enum DiffType {
    /// Speed
    Speed,
    /// Aim
    Aim,
}

impl DiffType {
    #[inline]
    /// TODO: what is this value?
    pub fn decay_base(&self) -> f64 {
        match self {
            DiffType::Speed => 0.3,
            DiffType::Aim => 0.15,
        }
    }

    #[inline]
    /// TODO: what is this value?
    pub fn weight_scaling(&self) -> f64 {
        match self {
            DiffType::Speed => 1400.0,
            DiffType::Aim => 26.25,
        }
    }
}

/// Result from difficulty calculation
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DiffCalcOutput {
    /// Total star rating
    pub total_stars: f64,

    /// Aim stars
    pub aim_stars: f64,

    /// Speed stars
    pub speed_stars: f64,

    /// Number of notes that are considered singletaps by the difficulty calculator
    pub nsingles: u32,

    /// Number of taps slower or equal to the singletap threshold value
    pub nsingles_threshold: u32,
}

/// Errors that could occur during pp calculation
#[derive(Debug, Error)]
pub enum Error {
    /// Unimplemented mode for pp calculation
    #[error("diff calc isn't yet implemented for this mode")]
    UnimplementedMode,
}

impl<'a> DiffCalc<'a> {
    /// New
    pub fn new(beatmap: &'a Beatmap) -> Self {
        let mut hit_objects = Vec::new();
        for obj in beatmap.hit_objects.iter() {
            hit_objects.push(CalcHitObject {
                inner: obj,
                strains: ObjectStrains::default(),
                is_single: false,
                normpos: Point::new(0.0, 0.0),
                d_distance: 0.0,
                delta_time: 0.0,
                angle: None,
            });
        }

        DiffCalc {
            beatmap,
            hit_objects,
            strains: Vec::new(),
        }
    }

    ///  calculates difficulty and stores results in self.total, self.aim, self.speed,
    ///  self.nsingles, self.nsingles_threshold.
    ///
    ///  singletap_threshold is the smallest milliseconds interval that will be considered
    ///  singletappable, defaults to 125ms which is 240 bpm 1/2 ((60000 / 240) / 2)
    pub fn calc(
        mut self,
        mods: Mods,
        singletap_threshold: Option<f64>,
    ) -> Result<DiffCalcOutput, Error> {
        let singletap_threshold = singletap_threshold.unwrap_or(125.0);

        // non-normalized diameter where the small circle size buff starts
        const CIRCLESIZE_BUFF_THRESHOLD: f64 = 30.0;
        const STAR_SCALING_FACTOR: f64 = 0.0675; // global stars multiplier

        // 50% of the difference between aim and speed is added to star rating to compensate aim
        // only or speed only maps
        const EXTREME_SCALING_FACTOR: f64 = 0.5;

        const PLAYFIELD_WIDTH: f64 = 512.0; // in osu!pixels
        const PLAYFIELD_CENTER: Point<f64> =
            Point::new(PLAYFIELD_WIDTH / 2.0, PLAYFIELD_WIDTH / 2.0);

        if !matches!(self.beatmap.mode, Mode::Osu) {
            return Err(Error::UnimplementedMode);
        }

        // calculate CS with mods
        let ModsApply { speed_mul, cs, .. } = mods_apply(
            mods,
            0.0,
            0.0,
            self.beatmap.difficulty.circle_size as f64,
            0.0,
        );

        // circle radius
        let radius = (PLAYFIELD_WIDTH / 16.0) * (1.0 - 0.7 * (cs - 5.0) / 5.0);

        // positions are normalized on circle radius so that we can calc as if everything was the
        // same circlesize
        let mut scaling_factor = 52.0 / radius;

        // low cs buff (credits to osuElements)
        if radius < CIRCLESIZE_BUFF_THRESHOLD {
            scaling_factor *= 1.0 + (CIRCLESIZE_BUFF_THRESHOLD - radius).min(5.0) / 50.0;
        }

        let playfield_center = PLAYFIELD_CENTER * scaling_factor;

        // calculate normalized positions
        for curr_idx in 0..self.hit_objects.len() {
            let (a, b) = self.hit_objects.split_at_mut(curr_idx);
            let obj = &mut b[0];

            if let HitObjectKind::Spinner(_) = obj.inner.kind {
                obj.normpos = playfield_center.clone();
            } else {
                let pos = Point::new(obj.inner.pos.x as f64, obj.inner.pos.y as f64);
                obj.normpos = pos * scaling_factor;
            }

            if curr_idx >= 2 {
                let prev2 = &a[curr_idx - 2];
                let prev1 = &a[curr_idx - 1];

                let v1: Point<f64> = prev2.normpos - prev1.normpos;
                let v2: Point<f64> = obj.normpos - prev1.normpos;
                let dot = v1.dot(v2);
                let det = v1.x * v2.y - v1.y * v2.x;
                obj.angle = Some(det.atan2(dot).abs());
            } else {
                obj.angle = None;
            }
        }

        // speed and aim stars
        let (mut speed_stars, speed_diff) = self.calc_individual(DiffType::Speed, speed_mul);
        let (mut aim_stars, aim_diff) = self.calc_individual(DiffType::Aim, speed_mul);

        fn length_bonus(star: f64, diff: f64) -> f64 {
            0.32 + 0.5 * ((diff + star).log10() - star.log10())
        }

        let aim_length_bonus = length_bonus(aim_stars, aim_diff);
        let speed_length_bonus = length_bonus(speed_stars, speed_diff);

        aim_stars = aim_stars.sqrt() * STAR_SCALING_FACTOR;
        speed_stars = speed_stars.sqrt() * STAR_SCALING_FACTOR;

        // touchscreen nerf
        if mods.contains(Mods::TouchDevice) {
            aim_stars = aim_stars.powf(0.8);
        }

        // total stars
        let mut total_stars = aim_stars + speed_stars;
        total_stars += (speed_stars - aim_stars).abs() * EXTREME_SCALING_FACTOR;

        let mut nsingles = 0;
        let mut nsingles_threshold = 0;
        for curr_idx in 1..self.hit_objects.len() {
            let obj = &self.hit_objects[curr_idx];
            let prev = &self.hit_objects[curr_idx - 1];

            if obj.is_single {
                nsingles += 1;
            }

            if !matches!(
                obj.inner.kind,
                HitObjectKind::Circle | HitObjectKind::Slider(_)
            ) {
                continue;
            }

            let interval = (obj.timef() - prev.timef()) / speed_mul;
            if interval >= singletap_threshold {
                nsingles_threshold += 1;
            }
        }

        Ok(DiffCalcOutput {
            speed_stars,
            aim_stars,
            total_stars,
            nsingles,
            nsingles_threshold,
        })
    }

    /// Calculates total strain for difftype. this assumes the normalized positions for hitobjects
    /// are already present
    pub fn calc_individual(&mut self, diff_type: DiffType, speed_mul: f64) -> (f64, f64) {
        // max strains are weighted from highest to lowest. this is how much the weight decays
        const DECAY_WEIGHT: f64 = 0.9;

        // strains are calculated by analyzing the map in chunks and taking the peak strains in
        // each chunk. this is the length of a strain interval in milliseconds
        let strain_step = 400.0 * speed_mul;

        self.strains.clear();
        // first object doesn't generate a strain so we begin with an incremented interval end
        let mut interval_end = (self.hit_objects[0].timef() / strain_step).ceil() * strain_step;
        let mut max_strain = 0.0f64;

        for curr_idx in 1..self.hit_objects.len() {
            let (a, b) = self.hit_objects.split_at_mut(curr_idx);
            let prev = &a[curr_idx - 1];
            let obj = &mut b[0];
            d_strain(diff_type, obj, prev, speed_mul);

            while obj.timef() > interval_end {
                // add max strain for this interval
                self.strains.push(max_strain);

                // decay last object's strains until the next interval and use that as the initial
                // max strain
                let decay = diff_type
                    .decay_base()
                    .powf((interval_end - prev.timef()) / 1000.0);

                max_strain = prev.strains[diff_type] * decay;
                interval_end += strain_step;
            }

            max_strain = max_strain.max(obj.strains[diff_type]);
        }

        // don't forget to add the last strain
        self.strains.push(max_strain);

        // weigh the top strains sorted from highest to lowest
        let mut weight = 1.0;
        let mut total = 0.0;
        let mut difficulty = 0.0;

        let mut strains = self.strains.clone();
        strains.sort_by_key(|k| Reverse(NotNan::new(*k).unwrap()));
        for strain in strains.iter() {
            total += strain.powf(1.2);
            difficulty += strain * weight;
            weight *= DECAY_WEIGHT;
        }

        (difficulty, total)
    }
}

#[derive(Copy, Clone, Default, Debug)]
/// Strains
pub struct ObjectStrains {
    speed_strain: f64,
    aim_strain: f64,
}

impl Index<DiffType> for ObjectStrains {
    type Output = f64;
    fn index(&self, idx: DiffType) -> &Self::Output {
        match idx {
            DiffType::Speed => &self.speed_strain,
            DiffType::Aim => &self.aim_strain,
        }
    }
}

impl IndexMut<DiffType> for ObjectStrains {
    fn index_mut(&mut self, idx: DiffType) -> &mut Self::Output {
        match idx {
            DiffType::Speed => &mut self.speed_strain,
            DiffType::Aim => &mut self.aim_strain,
        }
    }
}

/// calculates the difftype strain value for a hitobject. stores the result in
/// `obj.strains[difftype]`
///
/// this assumes that normpos is already computed
pub fn d_strain(
    diff_type: DiffType,
    obj: &mut CalcHitObject,
    prev_obj: &CalcHitObject,
    speed_mul: f64,
) {
    let mut value = 0.0;
    let time_elapsed = (obj.timef() - prev_obj.timef()) / speed_mul;
    obj.delta_time = time_elapsed;
    let decay = diff_type.decay_base().powf(time_elapsed / 1000.0);

    if matches!(
        obj.inner.kind,
        HitObjectKind::Circle | HitObjectKind::Slider(_)
    ) {
        let distance = obj.normpos.distance(prev_obj.normpos);
        obj.d_distance = distance;
        let (new_value, is_single) = d_spacing_weight(
            diff_type,
            distance,
            time_elapsed,
            prev_obj.d_distance,
            prev_obj.delta_time,
            obj.angle,
        );
        value = new_value * diff_type.weight_scaling();
        if let DiffType::Speed = diff_type {
            obj.is_single = is_single;
        }
    }

    obj.strains[diff_type] = prev_obj.strains[diff_type] * decay + value;
    eprintln!("strain = {} * {} + {} = {}", prev_obj.strains[diff_type], decay, value, obj.strains[diff_type]);
}

/// calculates spacing weight and returns (weight, is_single)
///
/// NOTE: is_single is only computed for DIFF_SPEED
pub fn d_spacing_weight(
    diff_type: DiffType,
    distance: f64,
    delta_time: f64,
    prev_distance: f64,
    prev_delta_time: f64,
    angle: Option<f64>,
) -> (f64, bool) {
    use std::f64::consts::PI;
    const MIN_SPEED_BONUS: f64 = 75.0; // ~200BPM 1/4 streams
    const MAX_SPEED_BONUS: f64 = 45.0; // ~330BPM 1/4 streams
    const ANGLE_BONUS_SCALE: f64 = 90.0;
    const AIM_TIMING_THRESHOLD: f64 = 107.0;
    const SPEED_ANGLE_BONUS_BEGIN: f64 = 5.0 * PI / 6.0;
    const AIM_ANGLE_BONUS_BEGIN: f64 = PI / 3.0;

    // arbitrary thresholds to determine when a stream is spaced enough that it becomes hard to
    // alternate
    const SINGLE_SPACING: f64 = 125.0;

    let strain_time = delta_time.max(50.0);
    let prev_strain_time = prev_delta_time.max(50.0);

    match diff_type {
        DiffType::Aim => {
            let mut result = 0.0;
            if let Some(angle) = angle {
                if angle > AIM_ANGLE_BONUS_BEGIN {
                    let angle_bonus = ((prev_distance - ANGLE_BONUS_SCALE).max(0.0)
                        * (angle - AIM_ANGLE_BONUS_BEGIN).sin().powi(2)
                        * (distance - ANGLE_BONUS_SCALE).max(0.0))
                    .sqrt();
                    result = 1.5 * angle_bonus.max(0.0).powf(0.99)
                        / prev_strain_time.max(AIM_TIMING_THRESHOLD);
                }
            }

            let weighted_distance = distance.powf(0.99);
            let res = (result + weighted_distance / strain_time.max(AIM_TIMING_THRESHOLD))
                .max(weighted_distance / strain_time);
            (res, false)
        }

        DiffType::Speed => {
            let is_single = distance > SINGLE_SPACING;
            let distance = distance.min(SINGLE_SPACING);
            let delta_time = delta_time.max(MAX_SPEED_BONUS);
            let mut speed_bonus = 1.0;
            if delta_time < MIN_SPEED_BONUS {
                speed_bonus += ((MIN_SPEED_BONUS - delta_time) / 40.0).powi(2);
            }
            let mut angle_bonus = 1.0;
            if let Some(angle) = angle {
                if angle < SPEED_ANGLE_BONUS_BEGIN {
                    let s = (1.5 * (SPEED_ANGLE_BONUS_BEGIN - angle)).sin();
                    angle_bonus += s * s / 3.57;
                    if angle < PI / 2.0 {
                        angle_bonus = 1.28;
                        if distance < ANGLE_BONUS_SCALE && angle < PI / 4.0 {
                            angle_bonus += (1.0 - angle_bonus)
                                * ((ANGLE_BONUS_SCALE - distance) / 10.0).min(1.0);
                        } else if distance < ANGLE_BONUS_SCALE {
                            angle_bonus += (1.0 - angle_bonus)
                                * ((ANGLE_BONUS_SCALE - distance) / 10.0).min(1.0)
                                * ((PI / 2.0 - angle) * 4.0 / PI).sin();
                        }
                    }
                }
            }

            let res = ((1.0 + (speed_bonus - 1.0) * 0.75)
                * angle_bonus
                * (0.95 + speed_bonus * (distance / SINGLE_SPACING).powf(3.5)))
                / strain_time;
            (res, is_single)
        }
    }
}
