struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let first = *nums.get(0).unwrap();

        if nums.len() == 1 {
            return first;
        }

        if nums.len() == 2 {
            let second = *nums.get(1).unwrap();
            return if first > second { first } else { second };
        }

        let mut first = Some(*nums.get(0).unwrap());
        let mut second = None;
        let mut third = None;

        for i in nums {
            if i == first.unwrap() || i == second.unwrap() || i == third.unwrap() {
                continue;
            }

            if i >= first.unwrap() {
                third = second;
                second = first;
                first = i;
            } else if i >= second {
                third = second;
                second = i;
            } else if i >= third {
                third = i;
            }
        }

        if third == i32::MIN {
            first
        } else {
            third
        }
    }
}
