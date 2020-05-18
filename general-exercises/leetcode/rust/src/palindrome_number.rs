#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    /// Follow up implementation without `String` conversion
    /// - Reverse the digits of `x` and compare to `x`
    /// - Negative numbers are never palindromic
    pub fn is_palindrome(x: i32) -> bool {
        // Check for -ve `x` *prior* to performing the reverse *digit* check
        // as it would give a false positive for negative numbers e.g.
        // `reverse(-121) == -121`. See test cases below.
        if x < 0 {
            false
        } else {
            Solution::reverse(x).map_or(false, |reversed| x == reversed)
        }
    }

    /// Reverse the digits of `x`. Returns `None` on overflow.
    ///
    /// Adapted from previous solution to "Reverse Integer" problem.
    fn reverse(x: i32) -> Option<i32> {
        let mut x_remaining = x; // Each iteration removes a digit
        let mut result: i32 = 0;

        while x_remaining != 0 {
            let digit = x_remaining % 10;
            // Remove digit
            x_remaining /= 10;
            // Returns `None` early on overflow
            result = result.checked_mul(10).and_then(|x| x.checked_add(digit))?;
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn single_digit() {
        assert_eq!(Solution::is_palindrome(0), true);
    }

    #[test]
    fn reverse_simple() {
        assert_eq!(Solution::reverse(123), Some(321));
    }

    #[test]
    fn reverse_negative() {
        // Note: This case is avoided by `Solution::is_palindrome` by checking
        // for -ve input value.
        assert_eq!(Solution::reverse(-123), Some(-321));
    }

    #[test]
    fn reverse_leading_0() {
        assert_eq!(Solution::reverse(120), Some(21));
    }

    #[test]
    fn reverse_overflow_max() {
        assert_eq!(Solution::reverse(i32::max_value()), None);
    }

    #[test]
    fn reverse_overflow_min_returns() {
        assert_eq!(Solution::reverse(i32::min_value()), None);
    }
}
