struct Solution;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut count = 0;
        let mut map: HashMap<i32, i32> = std::collections::HashMap::with_capacity(nums.len());

        for i in nums {
            let count = map.entry(i).or_default();
            *count += 1;
        }

        for (key, val) in map {
            count += if val > 1 { 0 } else { key }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
        assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
