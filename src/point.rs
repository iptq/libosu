use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Point<T>(T, T);

impl<T: Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
