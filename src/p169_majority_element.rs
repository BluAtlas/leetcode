/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 *
 * https://leetcode.com/problems/majority-element/description/
 *
 * algorithms
 * Easy (63.91%)
 * Likes:    14521
 * Dislikes: 440
 * Total Accepted:    1.7M
 * Total Submissions: 2.7M
 * Testcase Example:  '[3,2,3]'
 *
 * Given an array nums of size n, return the majority element.
 *
 * The majority element is the element that appears more than ⌊n / 2⌋ times.
 * You may assume that the majority element always exists in the array.
 *
 *
 * Example 1:
 * Input: nums = [3,2,3]
 * Output: 3
 * Example 2:
 * Input: nums = [2,2,1,1,1,2,2]
 * Output: 2
 *
 *
 * Constraints:
 *
 *
 * n == nums.length
 * 1 <= n <= 5 * 10^4
 * -10^9 <= nums[i] <= 10^9
 *
 *
 *
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
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
