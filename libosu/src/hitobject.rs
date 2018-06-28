use Point;

#[derive(Debug)]
pub enum HitObjectKind {
    Circle,
    Slider,
    Spinner,
}

#[derive(Debug)]
pub struct HitObject {
    pub pos: Point<i32>,
    pub start_time: u32,
    pub kind: HitObjectKind,
}
