struct Solution;
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut ans = 0;
        let mut n = num;

        while n != 0 {
            n >>= 1;
            ans = (ans << 1) | 1;
        }

        num ^ ans
    }
}
