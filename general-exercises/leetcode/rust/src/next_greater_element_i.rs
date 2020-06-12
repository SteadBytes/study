pub struct Solution {}

impl Solution {
    /// Next Greater Number of a number `x` in `nums1` is the first greater
    /// number to the right of `x` *in* `nums2`.
    /// - Rephrased from problem description for clarity
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // Naive O(n^2) algorithm - not suitable for large input.
        nums1
            .iter()
            .map(|&x| {
                // Find position of `x` in `nums2`
                let pos = nums2.iter().position(|&y| y == x).unwrap();
                // Check all elements to the right of `x` in `nums2`
                if let Some(y) = nums2[pos + 1..].iter().find(|&&y| y > x) {
                    *y
                } else {
                    -1
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Macro for these tests

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }

    #[test]
    fn none_found() {
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![4, 2, 1]),
            vec![-1, -1]
        );
    }

    #[test]
    fn input_arrays_equal() {
        assert_eq!(
            Solution::next_greater_element(vec![5, 2, 4, 3, 7], vec![5, 2, 4, 3, 7]),
            vec![7, 4, 7, 7, -1]
        );
    }
}
