/*
 * @lc app=leetcode id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }

        let mut total_profit = 0;

        let mut stock_value_is_increasing = prices[0] < prices[1];
        let mut value_of_last_minima = prices[0];

        // loop over prices[1..n]
        for i in prices.windows(2) {
            match stock_value_is_increasing {
                true => {
                    if i[0] > i[1] {
                        stock_value_is_increasing = false;
                        total_profit += i[0] - value_of_last_minima;
                    }
                }
                false => {
                    if i[0] < i[1] {
                        stock_value_is_increasing = true;
                        value_of_last_minima = i[0];
                    }
                }
            };
        }

        // account for prices[last]
        if stock_value_is_increasing {
            total_profit += prices.last().unwrap() - value_of_last_minima;
        }

        total_profit
    }
}
// @lc code=end
