use HitObject;
use Mode;
use TimingPoint;

#[derive(Debug)]
pub struct BeatmapSet {}

#[derive(Debug)]
pub struct Difficulty {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
}

/// Represents a single beatmap.
#[derive(Debug)]
pub struct Beatmap<'map> {
    pub version: u32,

    pub audio_filename: String,
    pub audio_leadin: u32,
    pub preview_time: u32,
    pub countdown: bool,
    pub stack_leniency: f64,
    pub mode: Mode,

    pub hit_objects: Vec<HitObject>,
    pub timing_points: Vec<TimingPoint<'map>>,
}
