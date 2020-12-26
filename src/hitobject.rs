use std::cmp::Ordering;

use crate::{Additions, Point, SampleInfo, TimeLocation, TimingPoint};

/// Distinguishes between different types of slider splines.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum HitObjectKind {
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
        /// How long this slider is in pixels.
        pixel_length: f64,
        /// The number of milliseconds long that this slider lasts.
        duration: u32,
        /// Hitsounds on each repeat of the slider
        /// TODO: fix this
        edge_hitsounds: Vec<u32>,
        /// Additions on each repeat of the slider
        edge_additions: Vec<String>,
    },
    /// Spinner.
    Spinner {
        /// The time at which the slider ends.
        end_time: TimeLocation,
    },
}

/// Represents a single hit object.
#[derive(Clone, Debug)]
pub struct HitObject {
    /// The position on the map at which this hit object is located (head for sliders).
    pub pos: Point<i32>,
    /// When this hit object occurs during the map.
    pub start_time: TimeLocation,
    /// The kind of HitObject this represents (circle, slider, spinner).
    pub kind: HitObjectKind,
    /// Whether or not this object begins a new combo.
    pub new_combo: bool,
    /// Reference to the timing point under which this HitObject belongs.
    pub timing_point: Option<TimingPoint>,
    /// The number of combo colors to skip
    pub skip_color: i32,
    /// The hitsound additions attached to this hit object.
    pub additions: Additions,
    /// The sample used to play the hitsound assigned to this hit object.
    pub sample_info: SampleInfo,
}

impl Ord for HitObject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_time.cmp(&other.start_time)
    }
}

impl PartialOrd for HitObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HitObject {}

impl PartialEq for HitObject {
    fn eq(&self, other: &Self) -> bool {
        self.start_time == other.start_time
    }
}
