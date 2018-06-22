use Point;

pub enum HitObjectKind {
    Circle,
    Slider,
    Spinner,
}

pub struct HitObject {
    pos: Point<i32>,
    start_time: i32,
    end_time: i32,
    kind: HitObjectKind,
}
