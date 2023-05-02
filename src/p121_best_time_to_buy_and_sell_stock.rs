/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut p_i = prices[1] - prices[0]; // P(i) === max profit ending on the ith day
        let mut max = 0;
        if p_i > max {
            max = p_i
        }

        for i in 2..prices.len() {
            let case1 = p_i + (prices[i] - prices[i - 1]);
            let case2 = prices[i] - prices[i - 1];

            p_i = if case1 > case2 { case1 } else { case2 };

            if p_i > max {
                max = p_i
            }
        }

        return if max > 0 { max } else { 0 };
    }
}
// @lc code=end
