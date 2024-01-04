use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stk = Vec::with_capacity(s.len());

        let mut map = HashMap::with_capacity(3);
        map.insert(')', '(');
        map.insert(']', '[');
        map.insert('}', '{');

        for ch in s.chars() {
            if map.contains_key(&ch) {
                if stk.is_empty() {
                    return false;
                }

                if !stk.is_empty() && map.get(&ch).unwrap().eq(&stk.last().unwrap()) {
                    stk.pop();
                } else {
                    return false;
                }
            } else {
                stk.push(ch);
            }
        }

        stk.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(Solution::is_valid("]".into()), false);
        assert_eq!(Solution::is_valid("(".into()), false);
        assert_eq!(Solution::is_valid("(]".into()), false);
        assert_eq!(Solution::is_valid("(])".into()), false);
        assert_eq!(Solution::is_valid("[[[]".into()), false);
        assert_eq!(Solution::is_valid("()[]{}".into()), true)
    }
}
