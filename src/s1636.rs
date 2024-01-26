use std::convert::identity;

struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        // let mut ans = Vec::with_capacity(nums.len());
        let mut map = std::collections::HashMap::with_capacity(nums.len());

        for &i in nums.iter() {
            let val = map.entry(i).or_insert(0);
            *val += 1;
        }

        let mut ans = nums.clone();
        ans.sort_by(|&a, &b| {
            let a_freq = map.get(&a).unwrap();
            let b_freq = map.get(&b).unwrap();

            if a_freq == b_freq {
                b.cmp(&a)
            } else {
                a_freq.cmp(b_freq)
            }
        });

        ans
    }
}
