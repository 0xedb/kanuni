struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut count = 0;

        for i in nums {
            if i == 1 {
                count += 1;
                max = max.max(count);
            } else if count != 0 {
                count = 0
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![4]), 0);
        assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1);
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
