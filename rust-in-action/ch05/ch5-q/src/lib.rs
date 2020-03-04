//! # Q7 Number Format
//!
//! This crate provides an implementation of the [Q7 number format](https://en.wikipedia.org/wiki/Q_(number_format))
//! intended as a storage and data transfer type only - `Q7` values do not
//! direcly support any arithmetic operations.
//!
//! `Qm.n` notation:
//! - `Q` = Q number format designation
//! - `m` (optional) = number of bits used to designate the **two's compliment
//! integer portion**
//!   - Exlusive *or* inclusive of the sign bit
//!   - Assumed to be 0 or 1 if not specified
//! - `n` = number of bits used to designate the fractional portion
//!   - e.g. number of bits to the right of decimal point
//!
//! `Q7` therefore represents values with 1 sign bit and 7 fractional bits.

// TODO: Add support for other Q formats

/// Represents a fixed-point number in the Q7 format.
/// Note: During conversion out of bounds values are coerced to the maximum of
/// the `Q7` range:
///
/// ```
/// use qnum::Q7;
///
/// assert_eq!(Q7::from(10.), Q7::from(1.));
/// assert_eq!(Q7::from(-10.), Q7::from(-1.));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(x: f64) -> Self {
        // Out of bounds values coerce to the maximum of the range
        if x >= 1.0 {
            Q7(127)
        } else if x <= -1.0 {
            Q7(-128)
        } else {
            // 1. Multiply x by 2^n
            // 2. Round to nearest integer
            Q7((x * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(x: Q7) -> f64 {
        // 1. Convert x to floating point as if it were an integer
        // 2. Multiple by 2^-n
        (x.0 as f64) * 2f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(x: f32) -> Self {
        Q7::from(x as f64)
    }
}

impl From<Q7> for f32 {
    fn from(x: Q7) -> f32 {
        f64::from(x) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}
