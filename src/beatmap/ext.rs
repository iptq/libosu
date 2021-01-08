use crate::beatmap::Beatmap;
use crate::hitobject::{HitObject, HitObjectKind, SliderInfo, SpinnerInfo};
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
            HitObjectKind::Slider(SliderInfo { num_repeats, .. }) => {
                let duration = self.get_slider_duration(ho).unwrap();
                TimeLocation(ho.start_time.0 + (duration * num_repeats as f64) as i32)
            }
            HitObjectKind::Spinner(SpinnerInfo { end_time }) => end_time,
        }
    }

    /// Returns the slider duration for a given slider
    pub fn get_slider_duration(&self, ho: &HitObject) -> Option<f64> {
        let info = match &ho.kind {
            HitObjectKind::Slider(info) => info,
            _ => return None,
        };

        let slider_velocity = self.get_slider_velocity_at_time(ho.start_time);
        let slider_multiplier = self.difficulty.slider_multiplier;
        let pixels_per_beat = slider_multiplier * 100.0 * slider_velocity;
        let beats_number = info.pixel_length * info.num_repeats as f64 / pixels_per_beat;

        let bpm = self.get_bpm_at_time(ho.start_time)?;
        let beat_duration = 60_000.0 / bpm;
        let duration = beats_number * beat_duration;

        Some(duration)
    }

    /// Returns the slider velocity at the given time
    pub fn get_slider_velocity_at_time(&self, time: TimeLocation) -> f64 {
        // TODO: replace this with binary search
        let mut current = 1.0;

        // assume this is sorted
        for tp in self.timing_points.iter() {
            if tp.time > time {
                break;
            }

            match &tp.kind {
                TimingPointKind::Uninherited { .. } => {
                    current = 1.0;
                }
                TimingPointKind::Inherited { slider_velocity, .. } => {
                    current = *slider_velocity;
                }
            }
        }

        current
    }

    /// Returns the BPM at the given time
    pub fn get_bpm_at_time(&self, time: TimeLocation) -> Option<f64> {
        // TODO: replace this with binary search
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
