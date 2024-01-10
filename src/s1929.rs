struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut ans = vec![0; l * 2];

        for (i, &v) in nums.iter().enumerate() {
            ans[i] = v;
            ans[i + l] = v;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        );
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
