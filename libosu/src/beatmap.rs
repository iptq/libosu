use HitObject;
use Mods;
use TimingPoint;

pub struct BeatmapSet {}

pub struct Difficulty {
    HPDrainRateX: i32,
    CircleSizeX: i32,
    OverallDifficultyX: i32,
    ApproachRateX: i32,
}

pub struct Beatmap {
    pub Version: u32,

    pub AudioFilename: String,
    pub HitObjects: Vec<HitObject>,
    pub TimingPoints: Vec<TimingPoint>,
}

impl Difficulty {
    pub fn get_hp_drain_rate(&self, mods: Mods) -> f64 {
        let multiplier = 0.1;
        return self.HPDrainRateX as f64 * multiplier;
    }
}
