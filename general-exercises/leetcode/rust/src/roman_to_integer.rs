#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn roman_to_int(str: String) -> i32 {
        // Reading from right to left, subtractions occur iff the previous
        // numeral is less than the current one (see `subtractions` test case).
        let (_, sum) = str.chars().rfold((0, 0), |(prev, acc), c| {
            let v = numeral_value(c);
            if v >= prev {
                // Simple additive case e.g. "II"
                (v, acc + v)
            } else {
                // Subtractive case e.g. "IV"
                (v, acc - v)
            }
        });
        sum
    }
}

fn numeral_value(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        // TODO: Handle invalid inputs gracefully
        _ => unreachable!("Assuming well formed problem inputs!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_symbols() {
        assert_eq!(Solution::roman_to_int(String::from("I")), 1);
        assert_eq!(Solution::roman_to_int(String::from("V")), 5);
        assert_eq!(Solution::roman_to_int(String::from("X")), 10);
        assert_eq!(Solution::roman_to_int(String::from("L")), 50);
        assert_eq!(Solution::roman_to_int(String::from("C")), 100);
        assert_eq!(Solution::roman_to_int(String::from("D")), 500);
        assert_eq!(Solution::roman_to_int(String::from("M")), 1000);
    }

    #[test]
    // Note: These only occur when reading from right to left, the previous
    // numeral is < the current.
    fn subtractions() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);

        assert_eq!(Solution::roman_to_int(String::from("XL")), 40);
        assert_eq!(Solution::roman_to_int(String::from("XC")), 90);

        assert_eq!(Solution::roman_to_int(String::from("CD")), 400);
        assert_eq!(Solution::roman_to_int(String::from("CM")), 900);
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    /// ```
    /// MCMXCIV = 1994
    /// M = 1000
    /// CM = 900
    /// XC = 90
    /// IV = 4
    /// ```
    fn example_5() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
