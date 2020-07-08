#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match Self::lcp_index(&strs) {
            Some(i) => String::from(&strs[0][..i]),
            None => String::new(),
        }
    }

    /// If a longest common prefix exists in `strs`, returns the index of the
    /// character *after* it, otherwise returns `None`.
    fn lcp_index(strs: &Vec<String>) -> Option<usize> {
        // LCP cannot be any longer that the shortest String in `strs`
        let max_lcp_len = strs.iter().map(|s| s.len()).min()?;
        let mut chars: Vec<std::str::Chars> = strs.iter().map(|s| s.chars()).collect();
        // Find the first index where all the characters do not match
        (0..=max_lcp_len).find(|_| {
            let mut next_chars = chars.iter_mut().map(|cs| cs.next());
            // Safe to unwrap as this is called at most `max_lcp_len` times
            // so the full `chars` iterator is never exhausted.
            match next_chars.next().unwrap() {
                Some(first_ch) => {
                    !next_chars.all(|maybe_c| maybe_c.map_or(false, |c| c == first_ch))
                }
                _ => true,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! lcp_tests {
        ($($name:ident: $strs:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(Solution::longest_common_prefix($strs,), $expected)
                }
            )*
        };
    }

    lcp_tests! {
        example_1: vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ],
            String::from("fl"),

        example_2: vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ],
            String::from(""),

        single_char: vec![
                String::from("dog"),
                String::from("drag"),
                String::from("d")
            ],
            String::from("d"),

        entire_string: vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flowing")
            ],
            String::from("flow"),

        all_equal: vec![
                String::from("flow"),
                String::from("flow"),
                String::from("flow")
            ],
            String::from("flow"),

        all_equal_len: vec![
                String::from("flow"),
                String::from("drag"),
                String::from("this")
            ],
            String::from(""),

        empty_input: vec![], String::from(""),

        all_empty_strings: vec![
                String::from(""),
                String::from(""),
                String::from("")
            ],
            String::from(""),

        single_empty: vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("")
            ],
            String::from(""),
    }
}
