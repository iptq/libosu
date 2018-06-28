use Point;
use TimeLocation;

#[derive(Debug)]
pub enum HitObjectKind {
    Circle,
    Slider,
    Spinner,
}

#[derive(Debug)]
pub struct HitObject<'map> {
    pub pos: Point<i32>,
    pub start_time: TimeLocation<'map>,
    pub kind: HitObjectKind,
}
