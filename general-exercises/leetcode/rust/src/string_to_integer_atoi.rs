#![allow(dead_code)]

struct Solution {}

impl Solution {
    /// - Discard leading whitespace
    /// - Attempt to parse a number from the first non-whitespace character up
    /// - Characters after the integral number are *ignored*
    ///      - `1234 hello -> 1234`
    ///      - `1234hello -> 1234`
    ///      - `1234+5 -> 1234`
    /// - Return `0` if no valid conversion could be performed
    /// - Return min/max representable value if overflow occurs
    /// - Single leading `+` or `-` is ok
    ///   - `-2 -> -2`
    ///   - `+2 -> 2`
    ///   - `+-2 -> 0`, `-` following `+` cannot be parsed as an integer
    pub fn my_atoi(str: String) -> i32 {
        // TODO: For your own sanity, tidy this mess up.
        let mut negative = false;
        if let Some(mut s) = str.split_whitespace().next() {
            if s.starts_with("-") {
                s = &s[1..];
                negative = true;
            } else if s.starts_with("+") {
                s = &s[1..];
            };
            let digits: String = s.chars().take_while(|x| x.is_ascii_digit()).collect();
            if digits.len() == 0 {
                // Not a number -> unparseable
                0
            } else if let Ok(x) = digits.parse::<i32>() {
                x * if negative { -1 } else { 1 }
            } else if negative {
                // Underflow
                i32::min_value()
            } else {
                // Overflow
                i32::max_value()
            }
        } else {
            // Empty string after removing whitespace
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Example 1
    fn postitive() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
    }

    fn postitive_with_sign() {
        assert_eq!(Solution::my_atoi(String::from("+42")), 42);
    }

    fn postitive_leading_whitespace_ignored() {
        assert_eq!(Solution::my_atoi(String::from("  42")), 42);
    }

    #[test]
    /// Example 2
    fn negative() {
        assert_eq!(Solution::my_atoi(String::from("-42")), -42);
    }

    fn negative_leading_whitespace_ignored() {
        assert_eq!(Solution::my_atoi(String::from("  -42")), 42);
    }

    #[test]
    /// Example 3
    fn non_numeric_after_integral_number_ignored() {
        assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193);
    }

    #[test]
    /// Example 4
    fn unparseable_if_first_non_whitespace_is_non_numeric() {
        assert_eq!(Solution::my_atoi(String::from("words and 987")), 0);
    }

    #[test]
    /// Example 5
    fn underflow_returns_min_representable_value() {
        assert_eq!(
            Solution::my_atoi(String::from("-91283472332")),
            i32::min_value()
        );
    }

    #[test]
    fn overflow_returns_max_representable_value() {
        assert_eq!(
            Solution::my_atoi(String::from("2147483648")),
            i32::max_value()
        );
    }

    #[test]
    /// Problem is *not* parsing mathematical expressions, but following a
    /// strict set of parsing rules within which this is not allowed as the
    /// first character after the first sign must be numeric.
    fn unparseable_if_multiple_leading_signs() {
        assert_eq!(Solution::my_atoi(String::from("-+42")), 0);
        assert_eq!(Solution::my_atoi(String::from("+-42")), 0);
    }
}
