use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Sub};

use num::{cast, Float, NumCast};
use quickcheck::{Arbitrary, Gen};

/// Represents a 2D point (or any pair of objects).
#[allow(missing_docs)]
#[derive(
    Add, Sub, Mul, Div, Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq,
)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Create a new point
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T: Copy + NumCast> Point<T> {
    /// Converts this point to a floating point point
    #[inline]
    pub fn to_float<U: Float>(&self) -> Option<Point<U>> {
        Some(Point::new(cast(self.x)?, cast(self.y)?))
    }
}

impl<T: Float> Point<T> {
    /// Calculates the Euclidean distance between 2 points.
    #[inline]
    pub fn distance(&self, other: Point<T>) -> T {
        let dx = other.x.sub(self.x);
        let dy = other.y.sub(self.y);
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculates the magnitude of the vector.
    #[inline]
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculates the norm of the vector.
    #[inline]
    pub fn norm(&self) -> Point<T> {
        let m = self.magnitude();
        Point::new(self.x / m, self.y / m)
    }
}

impl<T: Arbitrary> Arbitrary for Point<T> {
    fn arbitrary(g: &mut Gen) -> Point<T> {
        Point::new(T::arbitrary(g), T::arbitrary(g))
    }
}
