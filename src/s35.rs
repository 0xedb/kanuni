struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len();

        while i < j {
            let mid = (i + j) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[mid] < target {
                i = mid + 1;
            } else {
                j = mid;
            }
        }

        i as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }
}
