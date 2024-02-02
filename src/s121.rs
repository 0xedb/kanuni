struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut i = 0;
        let mut j = 1;
        let mut ans = 0;
        let mut min = prices[0];

        while j < prices.len() {
            let buy = prices[i];
            let sell = prices[j];

            min = min.min(sell);
            ans = ans.max(sell - min);

            if sell > buy {
                ans = ans.max(sell - buy);
            } else {
                i += 1;
            }

            j += 1
        }

        ans
    }
}
