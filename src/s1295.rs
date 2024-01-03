struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|x| {
                let mut n = *x;
                let mut count = 0;

                while n != 0 {
                    n /= 10;
                    count += 1;
                }

                count % 2 == 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1)
    }
}
