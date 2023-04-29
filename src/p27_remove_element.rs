/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut removed_positions = std::collections::VecDeque::new();

        // walk up nums and record empty locations
        for i in 0..nums.len() {
            if nums[i] == val {
                removed_positions.push_back(i);
            }
        }

        // get total remaining elements, return if none removed
        let total = (nums.len() - removed_positions.len()) as i32;
        if removed_positions.len() == 0 {
            return total;
        }

        // fill empty locations front-to-back

        for i in (0..nums.len()).rev() {
            if *removed_positions.front().unwrap() >= total as usize {
                break;
            }
            if nums[i] != val {
                nums[removed_positions.pop_front().unwrap()] = nums[i];
                if removed_positions.len() == 0 {
                    break;
                }
            }
        }

        return total;
    }
}
// @lc code=end
