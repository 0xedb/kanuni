struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut dupe = Vec::with_capacity(nums.len());
        let mut ans = dupe.clone();

        for i in nums.into_iter() {
            dupe.push(i * i);
        }

        let mut i = 0;
        let mut k = dupe.len() - 1;

        while i <= k {
            let one = dupe.get(i);
            let two = dupe.get(k);

            if one >= two {
                ans.push(*one.unwrap());
                i += 1;
            } else {
                ans.push(*two.unwrap());
                k -= 1;
            }
        }

        ans.reverse();

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );

        assert_eq!(
            Solution::sorted_squares(vec![-5, -3, -2, -1]),
            vec![1, 4, 9, 25]
        );
    }
}
