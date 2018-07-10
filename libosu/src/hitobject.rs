use serde::ser::*;

use Point;
use TimeLocation;
use TimingPoint;

/// Distinguishes between different types of slider splines.
#[derive(Debug)]
pub enum SliderSplineKind {
    /// Linear is the most straightforward, and literally consists of two endpoints.
    Linear,
    /// Bezier is more complex, using control points to create smooth curves.
    Bezier,
    /// Catmull is a deprecated slider spline used mainly in older maps (looks ugly btw).
    Catmull,
    /// Perfect (circle) splines are circles circumscribed around three control points.
    Perfect,
}

/// Distinguishes between different types of hit objects.
#[derive(Debug)]
pub enum HitObjectKind<'map> {
    /// Regular hit circle.
    Circle,
    /// Slider.
    Slider {
        /// The algorithm used to calculate the spline.
        kind: SliderSplineKind,
        /// The control points that make up the body of the slider.
        control: Vec<Point<i32>>,
        /// The number of times this slider should repeat.
        repeats: u32,
    },
    /// Spinner.
    Spinner {
        /// The time at which the slider ends.
        end_time: TimeLocation<'map>,
    },
}

/// Represents a single hit object.
#[derive(Debug)]
pub struct HitObject<'map> {
    pub pos: Point<i32>,
    pub start_time: TimeLocation<'map>,
    pub kind: HitObjectKind<'map>,
    pub new_combo: bool,
    /// Reference to the timing point under which this HitObject belongs.
    pub timing_point: Option<&'map TimingPoint<'map>>,
    /// The number of combo colors to skip
    pub skip_color: i32,
    /// WIP
    pub hitsound: u32,
}

impl<'map> Serialize for HitObject<'map> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let state = serializer.serialize_struct("HitObject", 0)?;
        state.end()
    }
}
