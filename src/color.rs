use std::fmt;
use std::str::FromStr;

use crate::errors::ParseError;

/// Represents an RGB color.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
  /// red from 0-255
  pub red: u8,

  /// blue from 0-255
  pub green: u8,

  /// green from 0-255
  pub blue: u8,
}

impl Color {
  /// Create a new color from the respective parts
  pub fn new(red: u8, green: u8, blue: u8) -> Self {
    Color { red, green, blue }
  }
}

impl FromStr for Color {
  type Err = ParseError;

  fn from_str(line: &str) -> Result<Color, Self::Err> {
    let mut s = line.split(" : ");
    s.next().ok_or(ParseError::MissingColorComponent)?;
    let s = s.next().ok_or(ParseError::MissingColorComponent)?;
    let s = s.split(',').collect::<Vec<_>>();
    let red = s[0].parse::<u8>()?;
    let green = s[1].parse::<u8>()?;
    let blue = s[2].parse::<u8>()?;
    Ok(Color { red, green, blue })
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{},{},{}", self.red, self.green, self.blue)
  }
}
