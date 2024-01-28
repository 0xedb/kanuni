struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut k = 0;
        let mut j = nums.len() - 1;

        while i < j {
            if nums[i] == 0 {
                i += 1;
                k += 1;
            } else if nums[i] == 2 {
                nums.swap(i, j);
                j -= 1;
            } else {
                i += 1;
                nums.swap(i, k);
            }
        }
    }
}
