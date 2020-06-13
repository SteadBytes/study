use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    /// Next Greater Number (NGN) of a number `x` in `nums1` is the first greater
    /// number to the right of `x` *in* `nums2`.
    /// - Rephrased from problem description for clarity
    ///
    /// # Algorithm
    ///
    /// The NGN of any `x` is independent of `nums1`; NGNs are defined *only*
    /// by `nums2`. Therefore, NGNs can be determined first and the final
    /// result produced from a single pass of `nums1`.
    ///
    /// NGN of `nums2[i]` is `nums2[i + 1]` *iff* `nums2[i + 1] is greater
    /// then `nums2[i]`. Otherwise, the NGN of *both* `nums2[i]` and `nums2[i + 1]`
    /// (if present at all) is within `nums2[(i + 2)..]`. In other words, each
    /// element of `nums2` is either:
    /// - NGN of the previous element *only*
    /// - NGN of *all* elements in a preceding *decreasing* sequence
    /// - Not an NGN at all
    ///
    /// For example:
    ///
    /// ```text
    /// nums2 = [5, 2, 4, 3, 7]
    ///
    /// nums2[2] is the NGN of nums2[1]
    ///
    /// nums2[4] is the NGN of nums2[0], nums2[2] and nums2[3]
    /// - Decreasing sequence [5, 4, 3]
    ///
    /// nums2[1] is not an NGN at all
    /// ```
    ///
    /// Thus, a hashmap from elements of `nums2` to their NGNs can be built
    /// by maintaining a monotonically decreasing stack of values obtained by
    /// iterating through `nums2`. Elements are popped off the stack when the
    /// next value to be pushed is greater than the top of the stack; this next
    /// value is the NGN of the smaller values.
    ///
    /// Building the hashmap is `O(n)` - `n` iterations to loop over `nums2`
    /// and in the worst case `n - 1` iterations to pop all elements from the
    /// stack. Creating the final result is also `O(n)` - requiring a single
    /// pass through `nums1`. Thus the overall algorithm is `O(n)`.
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ngn = HashMap::new();
        let mut stack = Vec::new();

        // Pre-process `nums2` to populate hashmap of NGNs
        for x in nums2 {
            while !stack.is_empty() && stack.last().unwrap() < &x {
                ngn.insert(stack.pop().unwrap(), x);
            }
            stack.push(x);
        }

        // Lookup NGN for each el of `nums1` (-1 as default)
        nums1
            .iter()
            .map(|x| ngn.get(x).copied().unwrap_or(-1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // `nums1` and `nums2` are equal in these examples to demonstrate the
    // complete NGN mapping for `nums2`.
    #[test]
    fn complete_decreasing_sequence() {
        assert_eq!(
            Solution::next_greater_element(vec![7, 6, 5, 4, 8], vec![7, 6, 5, 4, 8]),
            vec![8, 8, 8, 8, -1]
        );
    }

    #[test]
    fn interleaved_decreasing_sequence() {
        assert_eq!(
            Solution::next_greater_element(vec![5, 2, 4, 3, 7], vec![5, 2, 4, 3, 7]),
            vec![7, 4, 7, 7, -1]
        );
    }
}
