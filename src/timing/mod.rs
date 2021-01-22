mod point;

use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use ordered_float::NotNan;

pub use self::point::*;

/// A struct representing a location in time as milliseconds (i32)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimestampMillis(pub i32);

impl TimestampMillis {
    /// Convert the timestamp to seconds
    pub fn as_seconds(&self) -> TimestampSec {
        TimestampSec(unsafe { NotNan::unchecked_new(self.0 as f64 / 1000.0) })
    }
}

impl fmt::Display for TimestampMillis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A struct representing a location in time as seconds (f64)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimestampSec(pub NotNan<f64>);

impl TimestampSec {
    /// Convert the timestamp to milliseconds
    pub fn as_millis(&self) -> TimestampMillis {
        TimestampMillis((self.0.into_inner() * 1000.0) as i32)
    }

    /// Create a new TimestampSec unsafely
    pub fn unsafe_new(time: f64) -> Self {
        TimestampSec(unsafe { NotNan::unchecked_new(time) })
    }
}

impl fmt::Display for TimestampSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Add<Duration> for TimestampSec {
    type Output = TimestampSec;

    fn add(self, other: Duration) -> Self::Output {
        TimestampSec(self.0 + other.0)
    }
}

impl Sub for TimestampSec {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        Duration(self.0 - other.0)
    }
}

/// Duration of time
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
pub struct Duration(pub NotNan<f64>);

impl Mul<f64> for Duration {
    type Output = Duration;

    fn mul(self, rhs: f64) -> Self::Output {
        Duration(self.0 * rhs)
    }
}

impl Div<f64> for Duration {
    type Output = Duration;

    fn div(self, rhs: f64) -> Self::Output {
        Duration(self.0 / rhs)
    }
}
