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
pub enum HitObjectKind<'map> {
    Circle,
    Slider {
        kind: SliderSplineKind,
        control: Vec<Point<i32>>,
    },
    Spinner {
        end_time: TimeLocation<'map>,
    },
}

#[derive(Debug)]
pub struct HitObject<'map> {
    pub pos: Point<i32>,
    pub start_time: TimeLocation<'map>,
    pub kind: HitObjectKind<'map>,
    pub new_combo: bool,
}
