mod point;

use std::{
  fmt,
  ops::{Add, Deref, Sub},
};

pub use self::point::*;

/// A struct representing a location in time as milliseconds (i32)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Millis(pub i32);

impl Millis {
  /// Converts from seconds to Milliseconds
  pub fn from_seconds(secs: f64) -> Millis {
    Millis((secs * 1000.0) as i32)
  }

  /// Converts this Milliseconds to seconds
  pub fn as_seconds(&self) -> f64 {
    self.0 as f64 / 1000.0
  }
}

impl fmt::Display for Millis {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_fmt(format_args!("{}ms", self.0))
  }
}

impl From<i32> for Millis {
  fn from(v: i32) -> Self {
    Self(v)
  }
}

impl Deref for Millis {
  type Target = i32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Add<Millis> for Millis {
  type Output = Millis;

  fn add(self, rhs: Millis) -> Self::Output {
    Millis(self.0 + rhs.0)
  }
}

impl Sub<Millis> for Millis {
  type Output = i32;

  fn sub(self, rhs: Millis) -> Self::Output {
    self.0 - rhs.0
  }
}
