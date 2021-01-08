use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};

use num::Float;

/// Represents a 2D point (or any pair of objects).
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Point<T>(pub T, pub T);

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
}

impl<T: PartialEq + Eq> Eq for Point<T> {}

impl<T: Hash> Hash for Point<T> {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(h);
        self.1.hash(h);
    }
}

impl<T: Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;
    fn sub(self, other: Point<T>) -> Self::Output {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Mul<Output = T>> Mul<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn mul(self, other: Point<T>) -> Self::Output {
        Point(self.0 * other.0, self.1 * other.1)
    }
}

impl<T: Clone + Mul<Output = T>> Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, other: T) -> Self::Output {
        Point(self.0 * other.clone(), self.1 * other)
    }
}

impl<T: Float> Point<T> {
    /// Calculates the Euclidean distance between 2 points.
    pub fn distance(&self, other: Point<T>) -> T {
        let dx = other.0.sub(self.0);
        let dy = other.1.sub(self.1);
        (dx * dx + dy * dy).sqrt()
    }
}
