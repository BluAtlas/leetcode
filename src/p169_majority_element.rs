/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut elm = nums[0];
        for i in 1..nums.len() {
            if nums[i] == elm {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                elm = nums[i];
                count = 1;
            }
        }
        return elm;
    }
}
// @lc code=end
