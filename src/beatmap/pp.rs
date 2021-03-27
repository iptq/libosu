//! Ported from pyttanko

use crate::beatmap::Beatmap;
use crate::enums::{Mode, Mods};

/// Results from pp calculation
pub struct PPCalc {
    total_pp: f64,
    aim_pp: f64,
    speed_pp: f64,
    acc_pp: f64,
    accuracy: f64,
}

/// Type of difficulty used for calculation
pub enum DiffType {
    /// Speed
    Speed,
    /// Aim
    Aim,
}

/// Score version
#[derive(Clone, Debug)]
pub enum ScoreVersion {
    /// Scorev1
    V1,
    /// Scorev2
    V2,
}

/// Errors that could occur during pp calculation
#[derive(Debug, Error)]
pub enum Error {
    /// Unimplemented mode for pp calculation
    #[error("pp calc isn't yet implemented for this mode")]
    UnimplementedMode,
}

/// Params to be passed to pp calc
#[derive(Clone, Debug)]
pub struct PPCalcParams {
    /// Max combo earned for the score to be calculated
    pub combo: u32,

    /// Number of 300s
    pub n300: u32,

    /// Number of 100s
    pub n100: u32,

    /// Number of 50s
    pub n50: u32,

    /// Number of misses
    pub nmiss: u32,

    /// Game mode
    pub mode: Mode,

    /// Mods used during this score
    pub mods: Mods,

    /// Score version
    pub score_version: ScoreVersion,
}

/// Difficulty and PP calculation
impl Beatmap {
    /// Calculate pp
    pub fn calculate_diff(&self) -> Result<PPCalc, Error> {
        self.calculate_diff_with_mods(Mods::None)
    }

    /// Calculate pp with mods
    pub fn calculate_diff_with_mods(&self, mods: Mods) -> Result<PPCalc, Error> {
        // non-normalized diameter where the small circle size buff starts
        const CIRCLESIZE_BUFF_THRESHOLD: f64 = 30.0;
        const STAR_SCALING_FACTOR: f64 = 0.0675; // global stars multiplier

        // 50% of the difference between aim and speed is added to star rating to compensate aim
        // only or speed only maps
        const EXTREME_SCALING_FACTOR: f64 = 0.5;

        if !matches!(self.mode, Mode::Osu) {
            return Err(Error::UnimplementedMode);
        }

        todo!()
    }

    /// Calculate individual
    pub fn calculate_individual(&self, diff_type: DiffType, speed_mul: f64) {}

    /// Calculate
    pub fn calculate_ppv2(&self, params: PPCalcParams) -> PPCalc {
        calculate_ppv2(
            0.0,
            0.0,
            self.difficulty.approach_rate as f64,
            self.difficulty.overall_difficulty as f64,
            0,
            0,
            0,
            0,
            params,
        )
    }
}

/// Calculates pp
pub fn calculate_ppv2(
    aim_stars: f64,
    speed_stars: f64,
    base_ar: f64,
    base_od: f64,
    max_combo: u32,
    nsliders: u32,
    ncircles: u32,
    nobjects: u32,
    params: PPCalcParams,
) -> PPCalc {
    // accuracy ---------------------------------------------------------------
    let accuracy = acc_calc(params.n300, params.n100, params.n50, params.nmiss);
    let mut real_acc = accuracy;
    let nspinners = nobjects - nsliders - ncircles;

    match params.score_version {
        ScoreVersion::V1 => {
            // scorev1 ignores sliders since they are free 300s for whatever reason it also ignores
            // spinners
            // TODO: wait does this actually work in practice? don't slider breaks take away from
            // actual 300s then?
            real_acc = acc_calc(
                params.n300 - nsliders - nspinners,
                params.n100,
                params.n50,
                params.nmiss,
            );

            // can go negative if we miss everything
            real_acc = real_acc.max(0.0);
        }
        ScoreVersion::V2 => {
            // TODO
        }
    }

    // global values ----------------------------------------------------------
    let nobjects_over_2k = nobjects as f64 / 2000.0;
    let mut length_bonus = 0.95 + 0.4 * nobjects_over_2k.min(1.0);
    if nobjects > 2000 {
        length_bonus += nobjects_over_2k.log10() * 0.5
    }

    let miss_penality_aim = 0.97
        * (1.0 - (params.nmiss as f64 / nobjects as f64).powf(0.775)).powi(params.nmiss as i32);
    let miss_penality_speed = 0.97
        * (1.0 - (params.nmiss as f64 / nobjects as f64).powf(0.775))
            .powf((params.nmiss as f64).powf(0.875));
    let combo_break = (params.combo as f64).powf(0.8) / (max_combo as f64).powf(0.8);

    // calculate stats with mods
    let ModsApply {
        speed_mul, ar, od, ..
    } = mods_apply(params.mods, base_ar, base_od, 0.0, 0.0);

    // ar bonus ---------------------------------------------------------------
    let mut ar_bonus = 0.0;
    if ar > 10.33 {
        ar_bonus += 0.4 * (ar - 10.33)
    } else if ar < 8.0 {
        ar_bonus += 0.01 * (8.0 - ar)
    }

    // aim pp -----------------------------------------------------------------
    let mut aim_pp = pp_base(aim_stars);
    aim_pp *= length_bonus;
    if params.nmiss > 0 {
        aim_pp *= miss_penality_aim;
    }
    aim_pp *= combo_break;
    aim_pp *= 1.0 + ar_bonus.min(ar_bonus * (nobjects as f64 / 1000.0));

    // hd bonus
    let hd_bonus = 1.0;
    if params.mods.contains(Mods::Hidden) {
        hd_bonus *= 1.0 + 0.04 * (12.0 - ar);
    }
    aim_pp *= hd_bonus;

    if params.mods.contains(Mods::Flashlight) {
        let mut fl_bonus = 1.0 + 0.35 * (nobjects as f64 / 200.0).min(1.0);
        if nobjects > 200 {
            fl_bonus += 0.3 * ((nobjects - 200) as f64 / 300.0).min(1.0);
        }
        if nobjects > 500 {
            fl_bonus += (nobjects - 500) as f64 / 1200.0;
        }
        aim_pp *= fl_bonus;
    }

    let acc_bonus = 0.5 + accuracy / 2.0;
    let od_squared = od * od;
    let od_bonus = 0.98 + od_squared / 2500.0;

    aim_pp *= acc_bonus;
    aim_pp *= od_bonus;

    // speed pp ---------------------------------------------------------------
    let speed_pp = pp_base(speed_stars);
    speed_pp *= length_bonus;
    if params.nmiss > 0 {
        speed_pp *= miss_penality_speed;
    }
    speed_pp *= combo_break;
    if ar > 10.33 {
        speed_pp *= 1.0 + ar_bonus.min(ar_bonus * (nobjects as f64 / 1000.0));
    }
    speed_pp *= hd_bonus;

    speed_pp *= (0.95 + od_squared / 750.0) * accuracy.powf((14.5 - od.max(8.0)) / 2.0);
    if params.n50 as f64 >= nobjects as f64 / 500.0 {
        speed_pp *= 0.98f64.powf((params.n50 - nobjects) as f64 / 500.0);
    }

    // acc pp -----------------------------------------------------------------
    let mut acc_pp = 1.52163f64.powf(od) * real_acc.powf(24.0) * 2.83;
    // length bonus (not the same as speed/aim length bonus)
    acc_pp = (ncircles as f64 / 1000.0).powf(0.3).min(1.15);

    if params.mods.contains(Mods::Hidden) {
        acc_pp *= 1.08;
    }

    if params.mods.contains(Mods::Flashlight) {
        acc_pp *= 1.02;
    }

    // total pp ---------------------------------------------------------------
    let mut final_multiplier = 1.12;

    if params.mods.contains(Mods::NoFail) {
        final_multiplier *= (1.0 - 0.2 * params.nmiss as f64).max(0.9);
    }

    if params.mods.contains(Mods::SpunOut) {
        final_multiplier *= 1.0 - (nspinners as f64 / nobjects as f64).powf(0.85);
    }

    let total_pp = (aim_pp.powf(1.1) + speed_pp.powf(1.1) + acc_pp.powf(1.1)).powf(1.0 / 1.1)
        * final_multiplier;

    PPCalc {
        total_pp,
        aim_pp,
        speed_pp,
        acc_pp,
        accuracy,
    }
}

/// Return the accuracy corresponding to the amount of 300s/100s/50s/misses
pub fn acc_calc(n300: u32, n100: u32, n50: u32, nmiss: u32) -> f64 {
    let total_objs = n300 + n100 + n50 + nmiss;
    if total_objs == 0 {
        return 0.0;
    }

    (n300 as f64 * 6.0 + n100 as f64 * 2.0 + n50 as f64 * 1.0) / (total_objs as f64 * 6.0)
}

struct ModsApply {
    speed_mul: f64,
    ar: f64,
    od: f64,
    cs: f64,
    hp: f64,
}

/// Apply mods to difficulty scores
pub fn mods_apply(mods: Mods, ar: f64, od: f64, cs: f64, hp: f64) -> ModsApply {
    const MODS_SPEED_CHANGING: Mods = Mods::DoubleTime | Mods::HalfTime | Mods::Nightcore;
    const MODS_MAP_CHANGING: Mods = Mods::HardRock | Mods::Easy | MODS_SPEED_CHANGING;

    if !mods.intersects(MODS_SPEED_CHANGING) {
        return ModsApply {
            speed_mul: 1.0,
            ar,
            od,
            cs,
            hp,
        };
    }

    let mut speed_mul = 1.0;

    if mods.intersects(Mods::DoubleTime | Mods::Nightcore) {
        speed_mul *= 1.5;
    }

    if mods.contains(Mods::HalfTime) {
        speed_mul *= 0.75;
    }

    let od_ar_hp_modifier = 1.0;

    if mods.contains(Mods::HardRock) {
        od_ar_hp_modifier = 1.4;
    } else if mods.contains(Mods::Easy) {
        od_ar_hp_modifier *= 0.5;
    }

    let ar = {
        const AR0_MS: f64 = 1800.0;
        const AR5_MS: f64 = 1200.0;
        const AR10_MS: f64 = 450.0;
        const AR_MS_STEP1: f64 = (AR0_MS - AR5_MS) / 5.0;
        const AR_MS_STEP2: f64 = (AR5_MS - AR10_MS) / 5.0;

        // convert AR into milliseconds
        let ar = ar * od_ar_hp_modifier;
        let mut ar_ms = if ar < 5.0 {
            AR0_MS - AR_MS_STEP1 * ar
        } else {
            AR5_MS - AR_MS_STEP2 * (ar - 5.0)
        };

        // stats must be capped to 0-10 before HT/DT which brings them to a range of -4.42-11.08
        // for OD and -5-11 for AR
        ar_ms = ar_ms.max(AR10_MS).min(AR0_MS);
        ar_ms /= speed_mul;

        // convert back to AR
        if ar_ms > AR5_MS {
            (AR0_MS - ar_ms) / AR_MS_STEP1
        } else {
            5.0 + (AR5_MS - ar_ms) / AR_MS_STEP2
        }
    };

    let od = {
        const OD0_MS: f64 = 80.0;
        const OD10_MS: f64 = 20.0;
        const OD_MS_STEP: f64 = (OD0_MS - OD10_MS) / 10.0;

        let od = od * od_ar_hp_modifier;
        let mut od_ms = OD0_MS - (OD_MS_STEP * od).ceil();
        od_ms = od_ms.max(OD10_MS).min(OD0_MS);
        od_ms /= speed_mul;
        (OD0_MS - od_ms) / OD_MS_STEP
    };

    let cs = {
        let mut cs = cs;
        if mods.contains(Mods::HardRock) {
            cs *= 1.3;
        } else if mods.contains(Mods::Easy) {
            cs *= 0.5;
        }
        cs.min(10.0)
    };

    let hp = (hp * od_ar_hp_modifier).min(10.0);

    ModsApply {
        speed_mul,
        ar,
        od,
        cs,
        hp,
    }
}

/// base pp value for stars, used internally by ppv2
pub fn pp_base(stars: f64) -> f64 {
    (5.0 * (stars / 0.0675).max(1.0) - 4.0).powf(3.0) / 100000.0
}
