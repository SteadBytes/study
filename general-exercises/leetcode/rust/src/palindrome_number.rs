#![allow(dead_code)]

struct Solution {}

impl Solution {
    /// Naive implementation:
    /// - Convert `x` to `String`
    /// - Check if the `String` is palindromic
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        // O(n/2)
        let mid = s.len() / 2;
        s.chars().take(mid).eq(s.chars().rev().take(mid))
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
}
