#![allow(dead_code)]
use std::cmp::max;

pub struct Solution {}

impl Solution {
    // TODO: Write out proof from handwritten notes
    pub fn max_area(height: Vec<i32>) -> i32 {
        assert!(height.len() >= 2, "n must be at least 2");

        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_area = 0;

        while l < r {
            let width = (r - l) as i32;
            if height[l] < height[r] {
                max_area = max(max_area, height[l] * width);
                l += 1;
            } else {
                max_area = max(max_area, height[r] * width);
                r -= 1;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Uses lines `(1, 0), (1, 8)` and `(9, 0), (9, 7)` to form a rectangle
    /// with corners `(1, 0), (1, 7), (9, 7), (9, 0)`
    #[test]
    fn example_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    /// Uses lines `(2, 0), (2, 7)` and `(8, 0), (8, 5)` to form a rectangle
    /// with corners `(2, 0), (2, 5), (8, 5), (8, 0)`
    #[test]
    fn another_example() {
        assert_eq!(Solution::max_area(vec![3, 7, 4, 9, 6, 8, 2, 5]), 30);
    }

    /// Uses lines `(1, 0), (1, 5)` and `(8, 0), (8, 5)` to form a rectangle
    /// with corners `(1, 0), (1, 5), (8, 5), (8, 0)`
    #[test]
    fn max_width() {
        assert_eq!(Solution::max_area(vec![5, 1, 1, 1, 1, 1, 1, 5]), 35);
    }

    /// Uses lines `(3, 0), (3, 20)` and `(5,0), (5, 20)` to form a rectangle
    /// with corners `(3, 0), (3, 20), (5, 20), (5, 0)`
    #[test]
    fn max_height() {
        assert_eq!(Solution::max_area(vec![5, 1, 20, 1, 20, 1, 1, 5]), 40);
    }

    /// Uses lines `(3, 0), (3, 20)` and `(5,0), (5, 30)` to form a rectangle
    /// with corners `(3, 0), (3, 20), (5, 20), (5, 0)`
    #[test]
    fn second_max_height() {
        assert_eq!(Solution::max_area(vec![5, 1, 20, 1, 30, 1, 1, 5]), 40);
    }
}
