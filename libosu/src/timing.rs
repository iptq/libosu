use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::Rc;

use num_rational::Ratio;
use serde::ser::*;

use SampleSet;

/// A struct representing a _precise_ location in time.
///
/// This enum represents a timestamp by either an absolute timestamp (milliseconds), or a tuple
/// (t, m, f) where _t_ is the `TimingPoint` that it's relative to, _m_ is the measure number
/// from within this timing section, and _f_ is a fraction (actually implemented with
/// `num_rational::Ratio`) that represents how far into the measure this note appears.
///
/// When possible, prefer to stack measures. The value of _f_ should not ever reach 1; instead, opt
/// to use measure numbers for whole amounts of measures. For example, 1 measure + 5 / 4 should be
/// represented as 2 measures + 1 / 4 instead.
#[derive(Clone, Debug)]
pub enum TimeLocation {
    /// Absolute timing in terms of number of milliseconds since the beginning of the audio file.
    /// Note that because this is an `i32`, the time is allowed to be negative.
    Absolute(i32),
    /// Relative timing based on an existing TimingPoint. The lifetime of this TimeLocation thus
    /// depends on the lifetime of the map.
    Relative {
        time: Box<TimeLocation>,
        bpm: f64,
        meter: u32,
        measures: u32,
        frac: Ratio<u32>,
    },
}

/// An enum distinguishing between inherited and uninherited timing points.
#[derive(Clone, Debug)]
pub enum TimingPointKind {
    /// Uninherited timing point
    Uninherited {
        /// BPM (beats per minute) of this timing section
        bpm: f64,
        /// The number of beats in a single measure
        meter: u32,
        /// List of inherited timing points that belong to this section.
        children: BTreeSet<Rc<TimingPoint>>,
    },
    /// Inherited timing point
    Inherited {
        /// The uninherited timing point to which this timing point belongs.
        /// This field is an option because parsing and tree-building occur in different stages.
        parent: Option<Rc<TimingPoint>>,
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

impl TimeLocation {
    /// Converts any `TimeLocation` into an absolute one.
    pub fn into_absolute(self) -> TimeLocation {
        TimeLocation::Absolute(self.into_milliseconds())
    }

    /// Converts any `TimeLocation` into an absolute time in milliseconds from the beginning of the
    /// audio file.
    pub fn into_milliseconds(&self) -> i32 {
        match self {
            TimeLocation::Absolute(ref val) => *val,
            TimeLocation::Relative {
                ref time,
                ref bpm,
                ref meter,
                measures: ref m,
                frac: ref f,
            } => {
                // the start of the previous timing point
                let base = time.into_milliseconds();

                // milliseconds per beat
                let mpb = 60_000.0 / *bpm;

                // milliseconds per measure
                let mpm = mpb * (*meter as f64);

                // amount of time from the timing point to the beginning of the current measure
                // this is equal to (milliseconds / measure) * (# measures)
                let measure_offset = mpm * (*m as f64);

                // this is the fractional part, from the beginning of the measure
                let remaining_offset = (*f.numer() as f64) * mpm / (*f.denom() as f64);

                // ok now just add it all together
                base + (measure_offset + remaining_offset) as i32
            }
        }
    }

    /// Converts any `TimeLocation` into a relative one.
    pub fn into_relative(self, tp: &TimingPoint) -> TimeLocation {
        let bpm = tp.get_bpm();
        let meter = tp.get_meter();
        let (measures, frac) = self.approximate(&tp.time, bpm, meter);
        TimeLocation::Relative {
            time: Box::new(tp.time.clone()),
            bpm,
            meter,
            measures,
            frac,
        }
    }

    /// Converts any `TimeLocation` into a relative time tuple given a `TimingPoint`.
    pub fn approximate(&self, time: &TimeLocation, bpm: f64, meter: u32) -> (u32, Ratio<u32>) {
        match self {
            TimeLocation::Absolute(ref val) => {
                // this is going to be black magic btw

                // in this function i'm going to assume that the osu editor is _extremely_
                // accurate, and all inaccuracies up to 2ms will be accommodated. this essentially
                // means if your timestamp doesn't fall on a beat exactly, and it's also _not_ 2ms
                // from any well-established snapping, it's probably going to fail horribly (a.k.a.
                // report large numbers for d)

                // oh well, let's give this a shot

                // first, let's calculate the measure offset
                // (using all the stuff from into_milliseconds above)
                let mpb = 60_000.0 / bpm;
                let mpm = mpb * (meter as f64);
                let base = time.into_milliseconds();
                let cur = *val;
                let measures = ((cur - base) as f64 / mpm) as i32;

                // approximate time that our measure starts
                let measure_start = base + (measures as f64 * mpm) as i32;
                let offset = cur - measure_start;

                // now, enumerate several well-established snappings
                let mut snappings = BTreeSet::new();
                for d in vec![1, 2, 3, 4, 6, 8, 12, 16] {
                    for i in 0..d {
                        let snap = (mpm * i as f64 / d as f64) as i32;
                        snappings.insert((i, d, snap));
                    }
                }

                // now find out which one's the closest
                let mut distances = snappings
                    .into_iter()
                    .map(|(i, d, n)| (i, d, (offset - n).abs()))
                    .collect::<Vec<_>>();
                distances.sort_unstable_by(|(_, _, n1), (_, _, n2)| n1.cmp(n2));

                // now see how accurate the first one is
                let (i, d, n) = distances.first().unwrap();
                if *n < 3 {
                    // yay accurate
                    return (measures as u32, Ratio::new(*i as u32, *d as u32));
                }

                // i'll worry about this later
                // this is probably going to just be some fraction approximation algorithm tho
                (0, Ratio::from(0))
            }
            TimeLocation::Relative {
                ref time,
                ref bpm,
                ref meter,
                ..
            } => {
                // need to reconstruct the TimeLocation because we could be using a different
                // timing point
                // TODO: if the timing point is the same, return immediately
                TimeLocation::Absolute(self.into_milliseconds()).approximate(
                    &*time.clone(),
                    *bpm,
                    *meter,
                )
            }
        }
    }
}

impl Eq for TimeLocation {}

impl PartialEq for TimeLocation {
    fn eq(&self, other: &TimeLocation) -> bool {
        self.into_milliseconds() == other.into_milliseconds()
    }
}

impl Ord for TimeLocation {
    fn cmp(&self, other: &TimeLocation) -> Ordering {
        self.into_milliseconds().cmp(&other.into_milliseconds())
    }
}

impl PartialOrd for TimeLocation {
    fn partial_cmp(&self, other: &TimeLocation) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TimingPoint {
    /// Gets the closest parent that is an uninherited timing point.
    pub fn get_uninherited_ancestor(&self) -> &TimingPoint {
        match &self.kind {
            &TimingPointKind::Uninherited { .. } => self,
            &TimingPointKind::Inherited { ref parent, .. } => match parent {
                Some(_parent) => _parent.get_uninherited_ancestor(),
                None => panic!("Inherited timing point does not have a parent."),
            },
        }
    }
    /// Gets the BPM of this timing section by climbing the timing section tree.
    pub fn get_bpm(&self) -> f64 {
        let ancestor = self.get_uninherited_ancestor();
        match &ancestor.kind {
            &TimingPointKind::Uninherited { ref bpm, .. } => *bpm,
            _ => panic!("The ancestor should always be an Uninherited timing point."),
        }
    }

    /// Gets the meter of this timing section by climbing the timing section tree.
    pub fn get_meter(&self) -> u32 {
        let ancestor = self.get_uninherited_ancestor();
        match &ancestor.kind {
            &TimingPointKind::Uninherited { ref meter, .. } => *meter,
            _ => panic!("The ancestor should always be an Uninherited timing point."),
        }
    }
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
