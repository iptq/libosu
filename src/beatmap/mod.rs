mod ext;

use anyhow::Result;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{Color, HitObject, HitObjectKind, Mode, SampleSet, TimeLocation, TimingPoint};

/// Difficulty settings defined by the map.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Difficulty {
    /// HP Drain Rate
    ///
    /// The wiki doesn't have a solid definition of this field yet.
    pub hp_drain_rate: f32,
    /// Circle Size
    ///
    /// This is a value between 0 and 10 representing how big circles should appear on screen.
    ///
    /// In osu!mania, this actually defines the number of columns (keys).
    pub circle_size: f32,
    /// Overall Difficulty
    pub overall_difficulty: f32,
    /// Approach Rate
    pub approach_rate: f32,
    /// Slider Multiplier
    pub slider_multiplier: f64,
    /// Slider tick rate
    pub slider_tick_rate: u32,
}

impl Difficulty {
    /// Calculates the size of a circle in OsuPixels, which is how big the circle appears on a
    /// 640x480 screen.
    ///
    /// The formula for this can be found [here][1] and is equal to `54.4 - 4.48 * cs`.
    ///
    /// [1]: https://osu.ppy.sh/wiki/en/Beatmapping/Circle_size
    pub fn circle_size_osupx(&self) -> f32 {
        54.4 - 4.48 * self.circle_size
    }

    /// Calculates the duration of time (in milliseconds) before the hit object's point of impact
    /// at which the object should begin fading in.
    ///
    /// The formula for this can be found [here][1] and is a piecewise function:
    ///
    /// - AR < 5: preempt = 1200ms + 600ms * (5 - AR) / 5
    /// - AR = 5: preempt = 1200ms
    /// - AR > 5: preempt = 1200ms - 750ms * (AR - 5) / 5
    ///
    /// [1]: https://osu.ppy.sh/wiki/en/Beatmapping/Approach_rate
    pub fn approach_preempt(&self) -> u32 {
        if self.approach_rate < 5.0 {
            1200 + (600.0 * (5.0 - self.approach_rate)) as u32 / 5
        } else if self.approach_rate == 5.0 {
            1200
        } else {
            1200 - (750.0 * (self.approach_rate - 5.0)) as u32 / 5
        }
    }

    /// Calculates the duration of time (in milliseconds) it takes the hitobject to fade in
    /// completely to 100% opacity.
    ///
    /// The formula for this can be found [here][1] and is a piecewise function:
    ///
    /// - AR < 5: fade_in = 800ms + 400ms * (5 - AR) / 5
    /// - AR = 5: fade_in = 800ms
    /// - AR > 5: fade_in = 800ms - 500ms * (AR - 5) / 5
    ///
    /// [1]: https://osu.ppy.sh/wiki/en/Beatmapping/Approach_rate
    pub fn approach_fade_time(&self) -> u32 {
        if self.approach_rate < 5.0 {
            800 + (400.0 * (5.0 - self.approach_rate)) as u32 / 5
        } else if self.approach_rate == 5.0 {
            800
        } else {
            800 - (500.0 * (self.approach_rate - 5.0)) as u32 / 5
        }
    }
}

/// Represents a single beatmap.
#[derive(Clone, Debug, PartialEq)]
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
    /// Returns the timing point associated with the timing section to which the given time belongs.
    pub fn locate_timing_point(&self, time: impl Into<TimeLocation>) -> Option<TimingPoint> {
        // TODO: make this efficient
        let mut tp = None;
        let time = time.into();
        for timing_point in self.timing_points.iter() {
            if timing_point.time < time {
                tp = Some(timing_point.clone());
            }
        }
        tp
    }

    /// Returns the hitobject located at the given time.
    pub fn locate_hitobject(&self, time: impl Into<TimeLocation>) -> Option<HitObject> {
        let time = time.into();
        for hit_object in self.hit_objects.iter() {
            if hit_object.start_time == time {
                return Some(hit_object.clone());
            }

            if let HitObjectKind::Slider { .. } = hit_object.kind {}
        }
        None
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
