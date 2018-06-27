use Point;

pub enum HitObjectKind {
    Circle,
    Slider,
    Spinner,
}

pub struct HitObject {
    pub pos: Point<i32>,
    pub start_time: u32,
    pub kind: HitObjectKind,
}
