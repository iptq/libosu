use std::rc::Rc;

pub struct BeatmapSet {}

pub struct Beatmap {
    set: Rc<BeatmapSet>,

    pub format: u32,
}
