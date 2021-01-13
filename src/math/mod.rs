mod point;

use std::marker::PhantomData;

use num::{cast, Float};

pub use self::point::Point;

/// Zero-sized struct for performing mathematical calculations on floating points.
#[derive(Default)]
pub struct Math<T>(PhantomData<T>);

impl<T: Float> Math<T> {
    /// Computes the circumcircle given 3 points.
    pub fn circumcircle(p1: Point<T>, p2: Point<T>, p3: Point<T>) -> (Point<T>, T) {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let (x3, y3) = (p3.x, p3.y);

        let two = num::cast::<_, T>(2.0).unwrap();
        let d = two.mul_add(x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2), T::zero());
        let ux = ((x1 * x1 + y1 * y1) * (y2 - y3)
            + (x2 * x2 + y2 * y2) * (y3 - y1)
            + (x3 * x3 + y3 * y3) * (y1 - y2))
            / d;
        let uy = ((x1 * x1 + y1 * y1) * (x3 - x2)
            + (x2 * x2 + y2 * y2) * (x1 - x3)
            + (x3 * x3 + y3 * y3) * (x2 - x1))
            / d;

        let center = Point::new(ux, uy);
        (center, center.distance(p1))
    }

    /// Get the point on the line segment on p1, p2 that ends after length
    #[allow(clippy::many_single_char_names)]
    pub fn point_on_line(a: Point<T>, b: Point<T>, len: T) -> Point<T> {
        let full = a.distance(b);
        let n = full - len;
        let x = (n * a.x + len * b.x) / full;
        let y = (n * a.y + len * b.y) / full;
        Point::new(x, y)
    }

    /// Checks if a, b, and c are all on the same line
    pub fn is_line(a: Point<T>, b: Point<T>, c: Point<T>) -> bool {
        ((b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)).abs() < cast(0.001).unwrap()
    }
}
