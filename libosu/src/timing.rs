/// A struct representing a _precise_ location in time.
///
/// This enum represents a timestamp by either an absolute timestamp (milliseconds), or a tuple
/// (t, m, d, i) where _t_ is the `TimingPoint` that it's relative to, _m_ is the measure number
/// from within this timing section, _d_ is a value representing the meter (for example, 0 =
/// 1/1 meter, 1 = 1/2 meter, 3 = 1/4 meter, etc.), and _i_ is the index from the start of the measure.
#[derive(Debug)]
pub enum TimeLocation<'map> {
    /// Absolute timing in terms of number of milliseconds since the beginning of the audio file.
    /// Note that because this is an `i32`, the time is allowed to be negative.
    Absolute(i32),
    /// Relative timing based on an existing TimingPoint. The lifetime of this TimeLocation thus
    /// depends on the lifetime of the map.
    Relative(&'map TimingPoint<'map>, u32, u32, u32),
}

#[derive(Debug)]
pub enum TimingPointKind {
    /// Uninherited timing point
    Uninherited {
        /// BPM (beats per minute) of this timing section
        bpm: f64,
    },
    /// Inherited timing point
    Inherited {
        /// Slider velocity multiplier
        slider_velocity: f64,
    },
}

#[derive(Debug)]
pub struct TimingPoint<'map> {
    /// The timestamp of this timing point, represented as a `TimeLocation`.
    pub time: TimeLocation<'map>,
    /// The type of this timing point. See `TimingPointKind`.
    pub kind: TimingPointKind,
}

impl<'map> TimeLocation<'map> {
    /// Converts any `TimeLocation` into an absolute time in milliseconds from the beginning of the
    /// audio file.
    pub fn into_milliseconds(&self) -> i32 {
        match self {
            TimeLocation::Absolute(ref val) => *val,
            TimeLocation::Relative(ref tp, ref m, ref d, ref i) => {
                let base = tp.time.into_milliseconds();
                base
            }
        }
    }

    /// Converts any `TimeLocation` into a relative time tuple given a `TimingPoint`.
    pub fn approximate(&self, tp: &'map TimingPoint) -> (u32, u32, u32) {
        match self {
            TimeLocation::Absolute(ref val) => (0, 0, 0),
            TimeLocation::Relative(ref tp, ref m, ref d, ref i) => {
                // need to reconstruct the TimeLocation because we could be using a different
                // timing point
                // TODO: if the timing point is the same, return immediately
                TimeLocation::Absolute(self.into_milliseconds()).approximate(tp)
            }
        }
    }
}
