use Point;
use TimeLocation;

#[derive(Debug)]
pub enum SliderSplineKind {
    Linear,
    Bezier,
    Catmull,
    Perfect,
}

#[derive(Debug)]
pub enum HitObjectKind {
    Circle,
    Slider(SliderSplineKind, Vec<Point<i32>>),
    Spinner,
}

#[derive(Debug)]
pub struct HitObject<'map> {
    pub pos: Point<i32>,
    pub start_time: TimeLocation<'map>,
    pub kind: HitObjectKind,
    pub new_combo: bool,
}
