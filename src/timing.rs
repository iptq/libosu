use std::cmp::Ordering;

use ordered_float::NotNan;
use serde::ser::*;

use crate::hitsounds::SampleSet;

/// A struct representing a location in time as milliseconds
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimestampMillis(pub i32);

impl TimestampMillis {
    /// Convert the timestamp to seconds
    pub fn as_seconds(&self) -> TimestampSec {
        TimestampSec(unsafe { NotNan::unchecked_new(self.0 as f64 / 1000.0) })
    }
}

/// A struct representing a location in time as seconds
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimestampSec(pub NotNan<f64>);

impl TimestampSec {
    /// Convert the timestamp to milliseconds
    pub fn as_milliseconds(&self) -> TimestampMillis {
        TimestampMillis((self.0.into_inner() * 1000.0) as i32)
    }
}

/// Info for uninherited timing point
#[derive(Clone, Debug)]
pub struct UninheritedTimingInfo {
    /// Milliseconds per beat (aka beat duration)
    pub mpb: f64,

    /// The number of beats in a single measure
    pub meter: u32,
}

/// Info for inherited timing point
#[derive(Clone, Debug)]
pub struct InheritedTimingInfo {
    /// Slider velocity multiplier
    pub slider_velocity: f64,
}

/// An enum distinguishing between inherited and uninherited timing points.
#[derive(Clone, Debug)]
pub enum TimingPointKind {
    /// Uninherited timing point
    Uninherited(UninheritedTimingInfo),

    /// Inherited timing point
    Inherited(InheritedTimingInfo),
}

/// A timing point, which represents configuration settings for a timing section.
///
/// This is a generic timing point struct representing both inherited and uninherited timing
/// points, distinguished by the `kind` field.
#[derive(Clone, Debug)]
pub struct TimingPoint {
    /// The timestamp of this timing point, represented as a `TimeLocation`.
    pub time: TimestampMillis,

    /// Whether or not Kiai time should be on for this timing point.
    pub kiai: bool,

    /// The sample set associated with this timing section.
    pub sample_set: SampleSet,

    /// Index (if using a custom sample)
    pub sample_index: u32,

    /// Volume of this timing section.
    pub volume: u16,

    /// The type of this timing point. See `TimingPointKind`.
    pub kind: TimingPointKind,
}

impl Eq for TimingPoint {}

impl PartialEq for TimingPoint {
    fn eq(&self, other: &TimingPoint) -> bool {
        self.time.eq(&other.time)
    }
}

impl Ord for TimingPoint {
    fn cmp(&self, other: &TimingPoint) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for TimingPoint {
    fn partial_cmp(&self, other: &TimingPoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Serialize for TimingPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let state = serializer.serialize_struct("TimingPoint", 0)?;
        state.end()
    }
}
