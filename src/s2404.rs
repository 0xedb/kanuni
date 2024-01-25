struct Solution;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut ans = 10i32.pow(5);
        let mut map = std::collections::HashMap::with_capacity(nums.len());

        for i in nums {
            if i % 2 == 0 {
                let val = map.entry(i).or_insert(0);
                *val += 1;

                max = *val.max(&mut max);
            }
        }

        if map.is_empty() {
            return -1;
        }

        for (k, v) in map {
            if v == max {
                ans = k.min(ans)
            }
        }

        ans
    }
}
