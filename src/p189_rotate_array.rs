/*
 * @lc app=leetcode id=189 lang=rust
 *
 * [189] Rotate Array
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let mut temp = vec![];
        for i in 0..k as usize {
            temp.push(nums[nums.len() - k + i])
        }
        for i in (k..nums.len()).rev() {
            nums[i] = nums[i - k];
        }
        for i in 0..temp.len() {
            nums[i] = temp[i];
        }
    }
}
// @lc code=end
