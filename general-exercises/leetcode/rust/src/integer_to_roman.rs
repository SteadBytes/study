#![allow(dead_code)]
use std::iter::repeat;

const NUMERALS: [&str; 13] = [
    "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
];

const NUMERAL_VALUES: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

struct Solution {}

impl Solution {
    /// Recursive algorithm:
    ///
    /// Given an integer `x`:
    /// - Find the largest numeral value, `v` that is `<= x`
    /// - Calculate `q = x / v, r = x % v`
    /// - Return `q * numeral_string` if `r = 0`
    /// - Otherwise, return `q * numeral_string + int_to_roman(r)`
    /// TODO: Implement with a `fold`? Since each recursion operates on an `x`
    /// strictly less than the previous step, the set of possible numerals
    /// is contains those strictly less than the numeral chosen in the previous
    /// step. I think, therefore, that this can be achieved with a single pass
    /// over the numerals to accumulate the result?
    pub fn int_to_roman(num: i32) -> String {
        // Recursion builds a `Vec` containg pairs of numeral `&str` and the
        // number of times the numeral is repeated. This is so that the final
        // result `String` can be allocated once, instead of concatenating
        // at each recursive step.
        Solution::build_numeral_repeat_pairs(num, vec![])
            .iter()
            .map(|(i, n)| repeat(*i).take(*n))
            .flatten()
            .collect()
    }

    fn build_numeral_repeat_pairs(x: i32, mut so_far: Vec<(&str, usize)>) -> Vec<(&str, usize)> {
        if x == 0 {
            return so_far;
        }

        let (i, v) = NUMERAL_VALUES
            .iter()
            .enumerate()
            .find(|(_, &v)| v <= x)
            .expect("Input must be in range 1..=3999");
        let q = x / v;
        let r = x % v;
        so_far.push((NUMERALS[i], q as usize));
        Solution::build_numeral_repeat_pairs(r, so_far)
    }
}

fn roman_value(x: i32) -> char {
    match x {
        1 => 'I',
        5 => 'V',
        10 => 'X',
        50 => 'L',
        100 => 'C',
        500 => 'D',
        1000 => 'M',
        // TODO: Handle invalid inputs gracefully
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_symbols() {
        assert_eq!(Solution::int_to_roman(1), String::from("I"));
        assert_eq!(Solution::int_to_roman(5), String::from("V"));
        assert_eq!(Solution::int_to_roman(10), String::from("X"));
        assert_eq!(Solution::int_to_roman(50), String::from("L"));
        assert_eq!(Solution::int_to_roman(100), String::from("C"));
        assert_eq!(Solution::int_to_roman(500), String::from("D"));
        assert_eq!(Solution::int_to_roman(1000), String::from("M"));
    }

    // #[test]
    // // Note: These only occur when reading from right to left, the previous
    // // numeral is < the current.
    // fn subtractions() {
    //     assert_eq!(Solution::int_to_roman(String::from("IV")), 4);
    //     assert_eq!(Solution::int_to_roman(String::from("IX")), 9);
    //
    //     assert_eq!(Solution::int_to_roman(String::from("XL")), 40);
    //     assert_eq!(Solution::int_to_roman(String::from("XC")), 90);
    //
    //     assert_eq!(Solution::int_to_roman(String::from("CD")), 400);
    //     assert_eq!(Solution::int_to_roman(String::from("CM")), 900);
    // }
    //

    /// ```text
    /// Largest divisible numeral = "I" = 1
    /// 3 / 1 = 3, no remainder
    /// 3 = 3 * "I" = "III"
    /// ```
    #[test]
    fn example_1() {
        assert_eq!(Solution::int_to_roman(3), String::from("III"));
    }

    #[test]
    /// ```text
    /// Largest divisible numeral = "IV" = 4
    /// 4 / 4 = 1, remainder 0
    /// 4 = 1 * "IV" = "IV"
    /// ```
    fn example_2() {
        assert_eq!(Solution::int_to_roman(4), String::from("IV"));
    }

    #[test]
    /// ```text
    /// Largest divisible numeral = "IX" = 9
    /// 9 / 9 = 1, remainder 0
    /// 9 = 1 * "IX" = "IX"
    /// ```
    fn example_3() {
        assert_eq!(Solution::int_to_roman(9), String::from("IX"));
    }

    #[test]
    /// ```text
    /// Largest divisible numeral = "L" = 50
    /// 58 / 50 = 1, remainder 8
    /// 58 = 1 * "L" + int_to_roman(8)
    ///   Largest divisible numeral = "V" = 5
    ///   8 / 5 = 1, remainder 3
    ///   8 = 1 * "V" + int_to_roman(3)
    ///     Greatest divisible numeral = "I" = 1
    ///     3 / 1 = 3, no remainder
    ///     3 = 3 * "I" = "III"
    ///   8 = "VIII"
    /// 58 = "LVIII"
    /// ```
    fn example_4() {
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
    }

    #[test]
    /// ```text
    /// Largest divisible numeral = "M" = 1000
    /// 1994 / 1000 = 1, remainder 994
    /// 1994 = 1 * "M" + int_to_roman(994)
    ///   Largest divisible numeral = "CM" = 900 <---+
    ///   994 / 900 = 1, remainder 94                |
    ///   994 = 1 * "CM" + int_to_roman(94)          |
    ///     Largest divisible numeral = "XC" = 90 <--+ Note use of subtraction
    ///     94 / 90 = 1, remainder 4                 |
    ///     94 = 1 * "XC" + int_to_roman(4)          |
    ///       Largest divisible numeral = "IV" = 4 <-+
    ///       4 / 4 = 1
    ///       4 = 1 * "IV" = "IV"
    ///     94 = "XCIV"
    ///   994 = "CMXCIV"
    /// 1994 = "MCMXCIV"
    /// ```
    fn example_5() {
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
    }
}
