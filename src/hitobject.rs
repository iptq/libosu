pub enum HitObjectKind {
    Circle,
    Slider,
    Spinner,
}

pub struct HitObject {
    kind: HitObjectKind,
}
