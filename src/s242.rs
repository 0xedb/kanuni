struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut one: Vec<_> = s.chars().collect();
        one.sort();

        let mut two: Vec<_> = t.chars().collect();
        two.sort();

        one == two
    }
}
