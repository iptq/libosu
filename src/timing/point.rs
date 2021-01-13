use std::cmp::Ordering;
use std::str::FromStr;
use std::fmt;

use serde::ser::*;

use crate::errors::ParseError;
use crate::hitsounds::SampleSet;
use crate::timing::TimestampMillis;

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

impl FromStr for TimingPoint {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<TimingPoint, Self::Err> {
        let parts = input.split(',').collect::<Vec<_>>();

        let timestamp = parts[0].parse::<i32>()?;
        let mpb = parts[1].parse::<f64>()?;
        let meter = parts[2].parse::<u32>()?;
        let sample_set = parts[3].parse::<i32>()?;
        let sample_index = parts[4].parse::<u32>()?;
        let volume = parts[5].parse::<u16>()?;
        let inherited = parts[6].parse::<i32>()? == 0;
        let kiai = parts[7].parse::<i32>()? > 0;

        // calculate bpm from mpb
        let _ = 60_000.0 / mpb;
        let time = TimestampMillis(timestamp);

        let timing_point = TimingPoint {
            kind: if inherited {
                TimingPointKind::Inherited(InheritedTimingInfo {
                    slider_velocity: -100.0 / mpb,
                })
            } else {
                TimingPointKind::Uninherited(UninheritedTimingInfo { mpb, meter })
            },
            kiai,
            sample_set: match sample_set {
                0 => SampleSet::None,
                1 => SampleSet::Normal,
                2 => SampleSet::Soft,
                3 => SampleSet::Drum,
                _ => panic!("Invalid sample set '{}'.", sample_set),
            },
            sample_index,
            volume,
            time,
        };

        Ok(timing_point)
    }
}

impl fmt::Display for TimingPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inherited = match self.kind {
            TimingPointKind::Inherited { .. } => 0,
            TimingPointKind::Uninherited { .. } => 1,
        };

        let (beat_length, meter) = match self.kind {
            TimingPointKind::Inherited(InheritedTimingInfo {
                slider_velocity, ..
            }) => (-100.0 / slider_velocity, 0),
            TimingPointKind::Uninherited(UninheritedTimingInfo { mpb, meter, .. }) => (mpb, meter),
        };

        write!(
            f,
            "{},{},{},{},{},{},{},{}",
            self.time.0,
            beat_length,
            meter,
            self.sample_set as i32,
            self.sample_index,
            self.volume,
            inherited,
            if self.kiai { 1 } else { 0 },
        )
    }
}
