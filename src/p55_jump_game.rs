/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 *
 * https://leetcode.com/problems/jump-game/description/
 *
 * algorithms
 * Medium (38.93%)
 * Likes:    16076
 * Dislikes: 822
 * Total Accepted:    1.4M
 * Total Submissions: 3.6M
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * You are given an integer array nums. You are initially positioned at the
 * array's first index, and each element in the array represents your maximum
 * jump length at that position.
 *
 * Return true if you can reach the last index, or false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last
 * index.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum
 * jump length is 0, which makes it impossible to reach the last index.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * 0 <= nums[i] <= 10^5
 *
 *
 */

use crate::Solution;

// @lc code=start
impl Solution {
    // Walk forward, Keeping track of the current highest amount of jumps you could have at the current index.
    // If it hits 0 before the last index, false. Otherwise, true.
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut current_best_amount_of_jumps = nums[0];

        for (i, v) in nums.iter().enumerate() {
            if current_best_amount_of_jumps == 0 && *v == 0 && i != nums.len() - 1 {
                return false;
            }

            if current_best_amount_of_jumps < *v {
                current_best_amount_of_jumps = *v - 1
            } else {
                current_best_amount_of_jumps -= 1;
            }
        }
        return true;
    }
}
// @lc code=end
