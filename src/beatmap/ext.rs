use crate::beatmap::Beatmap;
use crate::hitobject::{HitObject, HitObjectKind};
use crate::timing::{TimeLocation, TimingPoint, TimingPointKind};

impl Beatmap {
    /// Iterate both hit objects and timing points
    pub fn double_iter(&self) -> DoubleIter {
        DoubleIter::new(self)
    }

    /// Computes the end time of the given hitobject
    pub fn get_hitobject_end_time(&self, ho: &HitObject) -> TimeLocation {
        match ho.kind {
            HitObjectKind::Circle => ho.start_time,
            HitObjectKind::Slider { num_repeats, .. } => {
                let duration = self.get_slider_duration(ho).unwrap();
                TimeLocation(ho.start_time.0 + (duration * num_repeats as f64) as i32)
            }
            HitObjectKind::Spinner { end_time } => end_time,
        }
    }

    /// Returns the slider duration for a given slider
    pub fn get_slider_duration(&self, ho: &HitObject) -> Option<f64> {
        let pixel_length = match ho.kind {
            HitObjectKind::Slider { pixel_length, .. } => pixel_length,
            _ => return None,
        };

        let slider_multiplier = self.difficulty.slider_multiplier;
        let bpm = self.get_bpm_at_time(ho.start_time)?;
        let beat_duration = 60_000.0 / bpm;

        let value = pixel_length / (100.0 * slider_multiplier) * beat_duration;
        Some(value)
    }

    /// Returns the BPM at the given time
    pub fn get_bpm_at_time(&self, time: TimeLocation) -> Option<f64> {
        let mut current = None;

        // assume this is sorted
        for tp in self.timing_points.iter() {
            if tp.time > time {
                break;
            }

            if let TimingPointKind::Uninherited { mpb, .. } = tp.kind {
                current = Some(60_000.0 / mpb);
            }
        }

        current
    }
}

/// An iterator over both hit objects and their corresponding timing points
pub struct DoubleIter<'a> {
    beatmap: &'a Beatmap,
    ho_index: usize,
    tp_index: usize,
}

impl<'a> DoubleIter<'a> {
    pub fn new(beatmap: &'a Beatmap) -> Self {
        DoubleIter {
            beatmap,
            ho_index: 0,
            tp_index: 0,
        }
    }
}

impl<'a> Iterator for DoubleIter<'a> {
    type Item = (&'a HitObject, &'a TimingPoint);

    fn next(&mut self) -> Option<Self::Item> {
        let ho = match self.beatmap.hit_objects.get(self.ho_index) {
            Some(v) => v,
            None => return None,
        };

        let tp = loop {
            let this_tp = match self.beatmap.timing_points.get(self.tp_index) {
                Some(v) => v,
                None => return None,
            };

            if let Some(v) = self.beatmap.timing_points.get(self.tp_index + 1) {
                if v.time < ho.start_time {
                    self.tp_index += 1;
                    continue;
                }
            }

            break this_tp;
        };

        self.ho_index += 1;
        Some((ho, tp))
    }
}
