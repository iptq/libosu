use Point;

pub enum HitObjectKind {
    Circle,
    Slider,
    Spinner,
}

pub struct HitObject {
    pos: Point<i32>,
    start_time: u32,
    end_time: u32,
    kind: HitObjectKind,
}
