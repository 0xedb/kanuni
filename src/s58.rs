struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        let mut was_space = false;

        for ch in s.chars() {
            if ch == ' ' {
                was_space = true;
            } else if was_space {
                count = 1;
                was_space = false
            } else {
                count += 1
            }
        }

        count
    }
}
