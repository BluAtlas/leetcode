/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return 1;
        }

        let mut i1 = 1;
        let mut i2 = 1;

        loop {
            if nums[i1 - 1] >= nums[i1] {
                i2 += 1;
                if i2 >= nums.len() {
                    nums.resize(i1, 0);
                    return i1 as i32;
                }
                while nums[i2] <= nums[i1 - 1] {
                    i2 += 1;
                    if i2 >= nums.len() {
                        nums.resize(i1, 0);
                        return i1 as i32;
                    }
                }
                nums[i1] = nums[i2];
            } else {
            }
            i1 += 1;
            if i1 >= nums.len() {
                nums.resize(i1, 0);
                return i1 as i32;
            }
        }
    }
}
// @lc code=end
