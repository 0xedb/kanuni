struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans = Vec::with_capacity(n as usize);

        for i in 1..=n {
            let mut response = String::with_capacity(8);

            if i % 3 == 0 {
                response.push_str("Fizz")
            }

            if i % 5 == 0 {
                response.push_str("Buzz")
            }

            if response.is_empty() {
                response = i.to_string();
            }

            ans.push(response);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizbuzz() {
        let ans = Solution::fizz_buzz(3);

        assert_eq!(ans, vec!["1", "2", "Fizz"])
    }
}
