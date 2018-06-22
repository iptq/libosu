pub struct BeatmapSet {}

pub struct Beatmap<'set> {
    set: &'set BeatmapSet,

    pub format: u32,
}
