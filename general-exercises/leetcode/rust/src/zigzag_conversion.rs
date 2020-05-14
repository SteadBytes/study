#![allow(dead_code)]

struct Solution {}

impl Solution {
    /// The zigzag has a period of `2 * num_rows - 1`, each input character
    /// can therefore be mapped onto an output row according to where it falls
    /// within this period when iterating through the input string from left
    /// to right; either up or down a row from the preceding character. This
    /// direction inverts half way through each period e.g. `num_rows - 1`.
    ///
    /// The original indices of the top and bottom rows will be
    /// `period * 2 * num_rows-1` and `(period * 2 * num_rows - 1) + num_rows - 1`
    /// respectively.
    ///
    /// Example 1 output (`n = 3`):
    ///
    /// ```text
    /// P   A   H   N
    /// A P L S I I G
    /// Y   I   R
    /// ```
    ///
    /// As input string indices:
    ///
    /// ```text
    /// 0   4   8     12
    /// 1 3 5 7 9  11 13
    /// 2   6   10
    /// ```
    ///
    /// Example 2 output (`n = 4`):
    ///
    /// ```text
    /// P     I    N
    /// A   L S  I G
    /// Y A   H R
    /// P     I
    /// ```
    ///
    /// As input string indices:
    ///
    /// ```text
    /// 0     6       12
    /// 1   5 7    11 13
    /// 2 4   8 10
    /// 3     9
    /// ```
    // TODO: Figure out how to do this entirely with iterators without
    // explicit Vec indexing/pushing
    pub fn convert(s: String, num_rows: i32) -> String {
        // Single row would not produce a zigzag at all
        if num_rows == 1 {
            return s;
        };
        let mut rows: Vec<Vec<char>> = vec![vec![]; num_rows as usize];
        let mut down = false;
        let mut current_row = 0;

        for c in s.chars().into_iter() {
            rows[current_row as usize].push(c);
            // Invert direction at start (down) or 1/2 way point (up) of period
            down ^= current_row == 0 || current_row == num_rows - 1;
            current_row += if down { 1 } else { -1 };
        }

        rows.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
    }

    #[test]
    fn single_row() {
        assert_eq!(Solution::convert(String::from("PA"), 1), String::from("PA"))
    }
}
