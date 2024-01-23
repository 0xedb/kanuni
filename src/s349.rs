struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let l = nums1.len().max(nums2.len());
        let mut ans = Vec::with_capacity(l);

        let mut set = std::collections::HashSet::with_capacity(l);

        for i in nums1 {
            set.insert(i);
        }

        for i in nums2 {
            if set.contains(&i) {
                ans.push(i);
                set.remove(&i);
            }
        }

        ans
    }
}
