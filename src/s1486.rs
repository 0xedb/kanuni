struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut ans = start + 2 * 0;

        for i in 1..n {
            ans ^= start + 2 * i
        }

        ans
    }
}
