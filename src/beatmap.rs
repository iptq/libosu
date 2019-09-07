use std::collections::BTreeSet;

use failure::Error;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    Color, HitObject, HitObjectKind, Hitsound, Mode, SampleSet, TimeLocation, TimingPoint,
};

/// Difficulty settings defined by the map.
#[derive(Debug, Default)]
pub struct Difficulty {
    /// HP Drain Rate
    ///
    /// The wiki doesn't have a solid definition of this field yet.
    pub hp_drain_rate: f32,
    /// Circle Size
    ///
    /// This is a value between 0 and 10 representing how big circles should appear on screen.
    /// The radius in osu!pixels is defined by the formula `32 * (1 - 0.7 * (CircleSize - 5) / 5)`, alternatively written `54.4 - 4.48 * CircleSize`.
    ///
    /// In osu!mania, this actually defines the number of columns (keys).
    pub circle_size: f32,
    /// Overall Difficulty
    pub overall_difficulty: f32,
    /// Approach Rate
    pub approach_rate: f32,
    /// Slider Multiplier
    pub slider_multiplier: f32,
}

/// Represents a single beatmap.
#[derive(Debug)]
pub struct Beatmap {
    /// The osu! file format being used
    pub version: u32,

    /// The name of the audio file to use, relative to the beatmap file.
    pub audio_filename: String,
    /// The amount of time (in milliseconds) added before the audio file begins playing. Useful for audio files that begin immediately.
    pub audio_leadin: u32,
    /// When (in milliseconds) the audio file should begin playing when selected in the song selection menu.
    pub preview_time: u32,
    /// Whether or not to show the countdown
    pub countdown: bool,
    /// The default sample set for hit objects which don't have a custom override.
    pub sample_set: SampleSet,
    /// Leniency for stacked objects.
    pub stack_leniency: f64,
    /// The game mode (standard, taiko, catch the beat, mania).
    pub mode: Mode,
    /// Whether or not to show black borders during breaks.
    pub letterbox_in_breaks: bool,
    /// TODO: unknown field
    pub widescreen_storyboard: bool,

    /// An instance of the difficulty settings.
    pub difficulty: Difficulty,

    /// Bookmarks in the editor
    pub bookmarks: Vec<i32>,
    /// The last setting used for distance spacing.
    pub distance_spacing: f64,
    /// The last setting used for beat divisor
    pub beat_divisor: u8,
    /// The last setting used for grid size
    pub grid_size: u8,
    /// The last setting used for timeline zoom
    pub timeline_zoom: f64,

    /// The title of the song (ASCII only).
    pub title: String,
    /// The title of the song (UTF-8).
    pub title_unicode: String,
    /// The artist of the song (ASCII only).
    pub artist: String,
    /// The artist of the song (UTF-8).
    pub artist_unicode: String,
    /// The creator of the mapset.
    pub creator: String,
    /// The name of the difficulty.
    pub difficulty_name: String,
    /// Optional source.
    pub source: String,
    /// Optional tags.
    pub tags: Vec<String>,
    /// The beatmap ID on Bancho.
    pub beatmap_id: i32,
    /// The beatmap set ID on Bancho.
    pub beatmap_set_id: i32,

    /// Overridden combo colors.
    pub colors: Vec<Color>,
    /// The set of hit objects.
    pub hit_objects: Vec<HitObject>,
    /// The set of timing points.
    pub timing_points: Vec<TimingPoint>,
}

impl Default for Beatmap {
    fn default() -> Self {
        Beatmap {
            version: 0,

            audio_filename: String::new(),
            audio_leadin: 0,
            preview_time: 0,
            countdown: false,
            sample_set: SampleSet::None,
            stack_leniency: 0.7,
            mode: Mode::Osu,
            letterbox_in_breaks: false,
            widescreen_storyboard: false,

            difficulty: Difficulty::default(),

            bookmarks: Vec::new(),
            distance_spacing: 0.0,
            beat_divisor: 1,
            grid_size: 1,
            timeline_zoom: 0.0,

            title: String::new(),
            title_unicode: String::new(),
            artist: String::new(),
            artist_unicode: String::new(),
            creator: String::new(),
            difficulty_name: String::new(),
            source: String::new(),
            tags: Vec::new(),
            beatmap_id: 0,
            beatmap_set_id: -1,

            colors: Vec::new(),
            hit_objects: Vec::new(),
            timing_points: Vec::new(),
        }
    }
}

impl Beatmap {
    pub(crate) fn associate_hitobjects(&mut self) {
        /*
        let mut curr = 1;
        for obj_ref in self.hit_objects.iter() {
            if curr >= self.timing_points.len() {
                break;
            }
            let obj = obj_ref.borrow();
            let obj_time = obj.start_time.into_milliseconds();
            // should we advance?
            let next_time = self.timing_points[curr].borrow().time.into_milliseconds();
            if obj_time >= next_time {
                curr += 1;
            }
            // assign timing point
            let tp = &self.timing_points[curr - 1].borrow();

            let bpm = tp.get_bpm();
            let meter = tp.get_meter();
            let (measures, frac) = obj.start_time.approximate(&tp.time, bpm, meter);
            let mut obj_mut = (**obj_ref).borrow_mut();
            obj_mut.start_time = TimeLocation::Relative {
                time: Box::new(tp.time.clone()),
                bpm: tp.get_bpm(),
                meter: tp.get_meter(),
                measures,
                frac,
            };
        }
        */
    }

    /// Returns the timing point associated with the timing section to which the given time belongs.`
    pub fn locate_timing_point(&self, time: impl Into<TimeLocation>) -> Option<TimingPoint> {
        // TODO: make this efficient
        let mut tp = None;
        let time = time.into();
        for timing_point in self.timing_points.iter() {
            if &timing_point.time < &time {
                tp = Some(timing_point.clone());
            }
        }
        tp
    }

    /// Returns the hitobject located at the given time.
    pub fn locate_hitobject(&self, time: impl Into<TimeLocation>) -> Option<HitObject> {
        let time = time.into();
        for mut hit_object in self.hit_objects.iter() {
            if &hit_object.start_time == &time {
                return Some(hit_object.clone());
            }

            if let HitObjectKind::Slider { .. } = hit_object.kind {}
        }
        None
    }

    /// Set a hitsound at the given time.
    // pub fn set_hitsound(&mut self, time: impl Into<TimeLocation>, hitsound: &Hitsound) {
    //     if let Some(hit_object) = self.locate_hitobject(time) {
    //         if let Some(mut hit_object) = self.hit_objects.take(&hit_object) {
    //             hit_object.set_hitsound(hitsound);
    //             self.hit_objects.insert(hit_object);
    //         }
    //     }
    // }

    /// Get a list of all hit objects.
    pub fn get_hitobjects(&self) -> Vec<HitObject> {
        self.hit_objects.iter().cloned().collect::<Vec<_>>()
    }

    /// Returns a list of this beatmap's hitsounds.
    ///
    /// This will also return hitsounds that occur on parts of objects, for example on slider
    /// bodies or slider ends. If a hitsound occurs on a spinner, the only "sound" that's counted
    /// is the moment that the spinner ends.
    pub fn get_hitsounds(&self) -> Result<Vec<(i32, Hitsound)>, Error> {
        let mut hitsounds = Vec::new();
        for obj in self.hit_objects.iter() {
            let start_time = obj.start_time.clone().as_milliseconds();
            match obj.kind {
                HitObjectKind::Slider {
                    ref repeats,
                    ref duration,
                    ..
                } => {
                    // TODO: calculate middle hitsounds
                    for i in 0..(repeats + 1) {
                        let time = start_time + (i * duration) as i32;
                        hitsounds.push((time, obj.hitsound.clone()));
                    }
                }
                _ => hitsounds.push((start_time, obj.hitsound.clone())),
            }
        }
        Ok(hitsounds)
    }
}

impl Serialize for Beatmap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Beatmap", 1)?;
        state.serialize_field("version", &self.version)?;

        // state.serialize_field("timing_points", &self.timing_points)?;
        state.end()
    }
}
