use std::cmp::Ordering;

use serde::ser::*;

use crate::hitsounds::SampleSet;

/// A struct representing a location in time.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeLocation(pub i32);

/// An enum distinguishing between inherited and uninherited timing points.
#[derive(Clone, Debug)]
pub enum TimingPointKind {
    /// Uninherited timing point
    Uninherited {
        /// Milliseconds per beat (aka beat duration)
        mpb: f64,
        /// The number of beats in a single measure
        meter: u32,
    },
    /// Inherited timing point
    Inherited {
        /// Slider velocity multiplier
        slider_velocity: f64,
    },
}

/// A timing point, which represents configuration settings for a timing section.
///
/// This is a generic timing point struct representing both inherited and uninherited timing
/// points, distinguished by the `kind` field.
#[derive(Clone, Debug)]
pub struct TimingPoint {
    /// The timestamp of this timing point, represented as a `TimeLocation`.
    pub time: TimeLocation,
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

/*
pub mod tests {
    extern crate lazy_static;

    #[allow(unused_imports)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    use super::*;

    lazy_static! {
        static ref TP: TimingPoint = TimingPoint {
            kind: TimingPointKind::Uninherited {
                bpm: 200.0,
                meter: 4,
                children: BTreeSet::new(),
            },
            time: TimeLocation::Absolute(12345),
            sample_set: SampleSet::None,
            sample_index: 0,
            volume: 100,
            kiai: false,
        };
        static ref ITP: TimingPoint = TimingPoint {
            kind: TimingPointKind::Inherited {
                parent: Some(&TP),
                slider_velocity: 0.0,
            },
            time: TimeLocation::Relative(&TP, 1, Ratio::from(0)),
            sample_set: SampleSet::None,
            sample_index: 0,
            volume: 80,
            kiai: false,
        };
    }

    pub fn get_test_data() -> Vec<(TimeLocation, i32)> {
        let test_data = vec![
            // uninherited timing points
            (TimeLocation::Relative(&TP, 0, Ratio::new(0, 1)), 12345), // no change from the measure at all
            (TimeLocation::Relative(&TP, 1, Ratio::new(0, 1)), 13545), // +1 measure (measure is 300ms, times 4 beats)
            (TimeLocation::Relative(&TP, 0, Ratio::new(1, 4)), 12645), // a single beat
            (TimeLocation::Relative(&TP, 0, Ratio::new(1, 2)), 12945), // half of a measure
            (TimeLocation::Relative(&TP, 0, Ratio::new(3, 4)), 13245), // 3 quarter notes
            // ok, on to inherited
            (TimeLocation::Relative(&ITP, 0, Ratio::new(0, 1)), 13545), // no change from the measure at all
            (TimeLocation::Relative(&ITP, 1, Ratio::new(0, 1)), 14745), // +1 measure, same as above
            (TimeLocation::Relative(&ITP, 0, Ratio::new(1, 4)), 13845), // a single beat
            (TimeLocation::Relative(&ITP, 0, Ratio::new(1, 2)), 14145), // half of a measure
            (TimeLocation::Relative(&ITP, 0, Ratio::new(3, 4)), 14445), // 3 quarter notes
        ];
        return test_data;
    }

    #[test]
    pub fn test_into_milliseconds() {
        let test_data = get_test_data();
        for (time, abs) in test_data.iter() {
            assert_eq!(time.into_milliseconds(), *abs);
        }
    }

    #[test]
    pub fn test_approximate() {
        let test_data = get_test_data();
        for (time, abs) in test_data.iter() {
            let t = TimeLocation::Absolute(*abs);
            match time {
                TimeLocation::Relative(tp, m, f) => {
                    let (m2, f2) = t.approximate(&tp);
                    assert_eq!((*m, *f), (m2, f2));
                }
                _ => panic!("This should never happen."),
            }
        }
    }
}
*/
