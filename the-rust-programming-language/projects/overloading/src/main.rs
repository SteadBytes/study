//! Demonstration of default generic type parameters via operator overloading
//! using the traits defined in `std::ops`

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point2D {
    x: i32,
    y: i32,
}

/// `std::ops:Add` has a default type parameter `rhs: RHS`
/// ```
/// trait Add<RHS=Self> {
///    type Output;
///
///    fn add(self, rhs: RHS) -> Self::Output;
/// }
/// ```
impl Add for Point2D {
    type Output = Point2D; // Associated type for `std::ops::Add`
    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[cfg(test)]
mod tests {

    mod point_2d {
        use crate::*;

        #[test]
        fn add_positive() {
            let p1 = Point2D { x: 1, y: 0 };
            let p2 = Point2D { x: 2, y: 3 };

            assert_eq!(p1 + p2, Point2D { x: 3, y: 3 });
        }

        #[test]
        fn add_negative() {
            let p1 = Point2D { x: 1, y: 0 };
            let p2 = Point2D { x: -1, y: 3 };

            assert_eq!(p1 + p2, Point2D { x: 0, y: 3 });
        }
    }

    mod millimeters {
        use crate::*;

        #[test]
        fn add_positive() {
            let x = Millimeters(100);
            let y = Meters(1);

            assert_eq!(x + y, Millimeters(1100));
        }
    }
}
