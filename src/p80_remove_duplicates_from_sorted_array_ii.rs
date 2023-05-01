/*
 * @lc app=leetcode id=80 lang=rust
 *
 * [80] Remove Duplicates from Sorted Array II
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }

        let mut i1 = 1;
        let mut i2 = 1;
        let mut last_count = 1;

        while i2 < nums.len() {
            if i1 >= nums.len() {
                break;
            }
            println!("{},{},{}", i1, i2, last_count);
            println!("{:?}", nums);
            println!("----------------");
            // if more than 2 in a row, change
            if last_count >= 2 && nums[i1 - 1] >= nums[i1] {
                while nums[i2] <= nums[i1 - 1] {
                    i2 += 1;
                    if i2 >= nums.len() {
                        nums.resize(i1, 0);
                        return i1 as i32;
                    }
                }
                nums[i1] = nums[i2];
                i2 += 1;
                last_count = 1;
            // less than 2 in a row, but should still be changed
            } else if last_count == 1 && (nums[i1 - 1] > nums[i1] || i2 == i1 + 1) {
                while nums[i2] < nums[i1 - 1] {
                    i2 += 1;
                    if i2 >= nums.len() {
                        nums.resize(i1, 0);
                        return i1 as i32;
                    }
                }
                nums[i1] = nums[i2];
                i2 += 1;
                if nums[i1] == nums[i1 - 1] {
                    last_count += 1;
                } else {
                    last_count = 1;
                }
            // not two in a row yet, keep going
            } else if nums[i1] == nums[i1 - 1] {
                last_count += 1;
            } else {
                last_count = 1;
            }
            i1 += 1;
        }
        nums.resize(i1, 0);
        return i1 as i32;
    }
}
// @lc code=end
