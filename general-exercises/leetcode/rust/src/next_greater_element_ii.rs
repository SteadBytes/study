pub struct Solution {}

impl Solution {
    /// Next Greater Number (NGN) of an element `x` in a **circular buffer**
    /// `nums` is the first greater number to its *traversing-order* next in
    /// the buffer.
    /// - Circular searching -> NGN may be before `x` in a standard (non-circular)
    ///   traversal of `nums`.
    ///
    /// # Brute Force Algorithm
    ///
    /// For each element `x` at index `i` in `nums`, perform a circular
    /// search from `nums[i] -> nums[i-1]`.
    /// - `x' > x` -> NGN = `x'`
    /// - Otherwise no NGN
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        debug_assert!(
            nums.len() <= 10000,
            "Problem description states nums.len() <= 10000"
        );

        nums.iter()
            .enumerate()
            .map(|(i, x)| {
                *nums[i..]
                    .iter()
                    .chain(nums[..i].iter())
                    .find(|&&y| y > *x)
                    .unwrap_or(&-1)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! nge_tests {
        ($($name:ident: $nums:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(Solution::next_greater_elements($nums), $expected);
                }
            )*
        };
    }

    nge_tests! {
        example_1: vec![1, 2, 1], vec![2, -1, 2],

        all_eq_no_ngns: vec![1, 1, 1], vec![-1, -1, -1],

        monotonic_increase_all_ngns_except_last:
            vec![1, 2, 3, 4, 5, 6],
            vec![2, 3, 4, 5, 6 ,-1],

        monotonic_decrease_all_ngns_except_first: vec![6, 5, 4, 3, 2, 1], vec![-1, 6, 6, 6, 6, 6],

        empty_input: vec![], vec![],
    }
}
