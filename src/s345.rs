use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut ans: Vec<_> = s.chars().collect();
        let vowels: HashSet<char> = HashSet::from_iter(['a', 'e', 'i', 'o', 'u']);

        let mut i = 0;
        let mut j = s.len() - 1;

        while i <= j {
            let iv = vowels.contains(&ans[i]);
            let jv = vowels.contains(&ans[j]);

            if iv && jv {
                ans.swap(i, j);
                i += 1;
                j -= 1;
            } else if !iv {
                i += 1;
            } else if !jv {
                j -= 1;
            }
        }

        ans.into_iter().collect()
    }
}