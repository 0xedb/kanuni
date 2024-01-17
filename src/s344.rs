struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let l = s.len();

        for i in 0..l / 2 {
            s.swap(i, l - 1 - i);
        }
    }
}
