use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (i, &v) in nums.iter().enumerate() {
            let target = target - v;

            if let Some(&x) = map.get(&target) {
                return vec![x, i as i32];
            }

            map.insert(v, i as i32);
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(Solution::two_sum(vec![3, 3,], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4, 6], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
