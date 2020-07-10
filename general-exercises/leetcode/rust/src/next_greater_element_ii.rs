pub struct Solution {}

impl Solution {
    /// Next Greater Number (NGN) of an element `x` in a **circular buffer**
    /// `nums` is the first greater number to its *traversing-order* next in
    /// the buffer.
    ///
    /// - Circular searching -> NGN may be before `x` in a standard (non-circular)
    ///   traversal of `nums`.
    ///
    /// # `O(n)` Stack-based Algorithm
    ///
    /// As in Next Greater Element I, the key observation is that `nums[i]` may
    /// be the NGN of a decreasing sub-sequence of elements *before* it in
    /// traversal order. However, due to the circular buffer in this case, 2
    /// standard traversals (a full circular traversal) are required to
    /// determine all NGNs.
    ///
    /// - `nums[i]` (in a non-circular traversal) could be the NGN for elements
    ///    in *both* `nums[..i]` and nums[i..n]`.
    ///
    /// Again, a stack representing a decreasing sub-sequence of elements is
    /// constructed. This time, however, the stack stores the *indices* of
    /// the sub-sequence (as opposed to the elements themselves) and is used
    /// to place the current value in a traversal into the correct position in
    /// the result vector.
    ///
    /// TODO: Is there a *functional* approach with equal/close efficiency?
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        debug_assert!(n <= 10000, "Problem description states nums.len() <= 10000");

        let process_candidate_ngn =
            |idx_stack: &mut Vec<usize>, result: &mut Vec<i32>, candidate: i32| {
                while let Some(&y) = idx_stack.last() {
                    if nums[y] < candidate {
                        result[idx_stack.pop().unwrap()] = candidate;
                    } else {
                        break;
                    }
                }
            };

        // Initialising to `-1` allows traversals to focus on *finding* NGNs;
        // overwriting as discovered
        let mut result: Vec<i32> = vec![-1; n];
        let mut idx_stack: Vec<usize> = vec![];

        // Fill NGN positions where a circular traversal is *not* needed e.g.
        // `nums[i]` is the NGN of one or more elements in `nums[0..i]`
        for (i, x) in nums.iter().enumerate() {
            process_candidate_ngn(&mut idx_stack, &mut result, *x);
            idx_stack.push(i);
        }

        // Fill NGN positions where a circular traversal *is* needed e.g.
        // `nums[i]` is the NGN of one or more elements in `nums[i..n]`
        for x in nums.iter() {
            process_candidate_ngn(&mut idx_stack, &mut result, *x);
        }
        result
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
