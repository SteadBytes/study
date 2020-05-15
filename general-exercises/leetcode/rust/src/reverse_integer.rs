#![allow(dead_code)]

struct Solution {}

impl Solution {
    /// Reverse the digits of an integer. Returs `0` if the result overflows.
    pub fn reverse(x: i32) -> i32 {
        let mut x_remaining = x; // Each iteration removes a digit
        let mut result: i32 = 0;

        while x_remaining != 0 {
            let digit = x_remaining % 10;
            // Remove digit
            x_remaining /= 10;
            result = result
                .checked_mul(10)
                .and_then(|x| x.checked_add(digit))
                .unwrap_or(0); // Exit loop on overflow
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn overflow_max_returns_0() {
        assert_eq!(Solution::reverse(i32::max_value()), 0);
    }

    #[test]
    fn overflow_min_returns_0() {
        assert_eq!(Solution::reverse(i32::min_value()), 0);
    }
}
