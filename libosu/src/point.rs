use std::fmt::{self, Display};
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub struct Point<T>(pub T, pub T);

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
