use HitObject;
use Mods;
use TimingPoint;

pub struct BeatmapSet {}

pub struct Difficulty {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
}

/// Represents a single beatmap.
pub struct Beatmap<'map> {
    pub version: u32,

    pub audio_filename: String,
    pub hit_objects: Vec<HitObject>,
    pub timing_points: Vec<TimingPoint<'map>>,
}
