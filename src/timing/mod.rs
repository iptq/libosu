mod point;

use std::fmt;
use std::ops::{Add,Sub};

use ordered_float::NotNan;

pub use self::point::*;

/// A struct representing a location in time as milliseconds (i32)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
pub struct TimestampSec(pub NotNan<f64>);

impl TimestampSec {
    /// Convert the timestamp to milliseconds
    pub fn as_millis(&self) -> TimestampMillis {
        TimestampMillis((self.0.into_inner() * 1000.0) as i32)
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

/// A struct representing a location in time as seconds (f64)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(pub NotNan<f64>);
