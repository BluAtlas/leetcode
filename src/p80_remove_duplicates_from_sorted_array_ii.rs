/*
 * @lc app=leetcode id=80 lang=rust
 *
 * [80] Remove Duplicates from Sorted Array II
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/description/
 *
 * algorithms
 * Medium (52.45%)
 * Likes:    4763
 * Dislikes: 982
 * Total Accepted:    502.3K
 * Total Submissions: 957.4K
 * Testcase Example:  '[1,1,1,2,2,3]'
 *
 * Given an integer array nums sorted in non-decreasing order, remove some
 * duplicates in-place such that each unique element appears at most twice. The
 * relative order of the elements should be kept the same.
 *
 * Since it is impossible to change the length of the array in some languages,
 * you must instead have the result be placed in the first part of the array
 * nums. More formally, if there are k elements after removing the duplicates,
 * then the first k elements of nums should hold the final result. It does not
 * matter what you leave beyond the first k elements.
 *
 * Return k after placing the final result in the first k slots of nums.
 *
 * Do not allocate extra space for another array. You must do this by modifying
 * the input array in-place with O(1) extra memory.
 *
 * Custom Judge:
 *
 * The judge will test your solution with the following code:
 *
 *
 * int[] nums = [...]; // Input array
 * int[] expectedNums = [...]; // The expected answer with correct length
 *
 * int k = removeDuplicates(nums); // Calls your implementation
 *
 * assert k == expectedNums.length;
 * for (int i = 0; i < k; i++) {
 * ⁠   assert nums[i] == expectedNums[i];
 * }
 *
 *
 * If all assertions pass, then your solution will be accepted.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,1,1,2,2,3]
 * Output: 5, nums = [1,1,2,2,3,_]
 * Explanation: Your function should return k = 5, with the first five elements
 * of nums being 1, 1, 2, 2 and 3 respectively.
 * It does not matter what you leave beyond the returned k (hence they are
 * underscores).
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,0,1,1,1,1,2,3,3]
 * Output: 7, nums = [0,0,1,1,2,3,3,_,_]
 * Explanation: Your function should return k = 7, with the first seven
 * elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
 * It does not matter what you leave beyond the returned k (hence they are
 * underscores).
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 3 * 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums is sorted in non-decreasing order.
 *
 *
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
