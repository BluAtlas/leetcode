/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 *
 * https://leetcode.com/problems/jump-game-ii/description/
 *
 * algorithms
 * Medium (39.88%)
 * Likes:    12233
 * Dislikes: 426
 * Total Accepted:    889.2K
 * Total Submissions: 2.2M
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * You are given a 0-indexed array of integers nums of length n. You are
 * initially positioned at nums[0].
 *
 * Each element nums[i] represents the maximum length of a forward jump from
 * index i. In other words, if you are at nums[i], you can jump to any nums[i +
 * j] where:
 *
 *
 * 0 <= j <= nums[i] and
 * i + j < n
 *
 *
 * Return the minimum number of jumps to reach nums[n - 1]. The test cases are
 * generated such that you can reach nums[n - 1].
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2. Jump
 * 1 step from index 0 to 1, then 3 steps to the last index.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,3,0,1,4]
 * Output: 2
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * 0 <= nums[i] <= 1000
 * It's guaranteed that you can reach nums[n - 1].
 *
 *
 */

use crate::Solution;

// @lc code=start
impl Solution {
    // for each index, check all possible jump locations, greedily choose the one that can jump the furthest
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut jump_count = 0;
        let mut current_index = 0;

        loop {
            let mut max_index = current_index;
            let mut max_index_value = nums[current_index] as usize;

            for potential_index in
                current_index + 1..(current_index + 1 + nums[current_index] as usize)
            {
                // occurs when potential index is the end, meaning we can jump to the end
                if potential_index >= nums.len() - 1 {
                    max_index = potential_index;
                    break;
                }

                let potential_index_value = nums[potential_index];
                let potential_index_jump_power = potential_index + potential_index_value as usize;

                if potential_index_jump_power >= max_index + max_index_value {
                    max_index = potential_index;
                    max_index_value = potential_index_value as usize;
                }
            }

            jump_count += 1;
            current_index = max_index;

            if current_index >= nums.len() - 1 {
                return jump_count as i32;
            }
        }
    }
}
// @lc code=end
