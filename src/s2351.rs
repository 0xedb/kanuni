use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut map = HashSet::with_capacity(s.len());

        for ch in s.chars() {
            if map.contains(&ch) {
                return ch;
            }

            map.insert(ch);
        }

        '\0'
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check() {
        assert_eq!(Solution::repeated_character("abcdd".into()), 'd');
        assert_eq!(Solution::repeated_character("abccbaacz".into()), 'c');
    }
}
