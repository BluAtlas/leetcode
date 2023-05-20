/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 *
 * https://leetcode.com/problems/merge-sorted-array/description/
 *
 * algorithms
 * Easy (46.75%)
 * Likes:    10153
 * Dislikes: 957
 * Total Accepted:    2M
 * Total Submissions: 4.2M
 * Testcase Example:  '[1,2,3,0,0,0]\n3\n[2,5,6]\n3'
 *
 * You are given two integer arrays nums1 and nums2, sorted in non-decreasing
 * order, and two integers m and n, representing the number of elements in
 * nums1 and nums2 respectively.
 *
 * Merge nums1 and nums2 into a single array sorted in non-decreasing order.
 *
 * The final sorted array should not be returned by the function, but instead
 * be stored inside the array nums1. To accommodate this, nums1 has a length of
 * m + n, where the first m elements denote the elements that should be merged,
 * and the last n elements are set to 0 and should be ignored. nums2 has a
 * length of n.
 *
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
 * Output: [1,2,2,3,5,6]
 * Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
 * The result of the merge is [1,2,2,3,5,6] with the underlined elements coming
 * from nums1.
 *
 *
 * Example 2:
 *
 *
 * Input: nums1 = [1], m = 1, nums2 = [], n = 0
 * Output: [1]
 * Explanation: The arrays we are merging are [1] and [].
 * The result of the merge is [1].
 *
 *
 * Example 3:
 *
 *
 * Input: nums1 = [0], m = 0, nums2 = [1], n = 1
 * Output: [1]
 * Explanation: The arrays we are merging are [] and [1].
 * The result of the merge is [1].
 * Note that because m = 0, there are no elements in nums1. The 0 is only there
 * to ensure the merge result can fit in nums1.
 *
 *
 *
 * Constraints:
 *
 *
 * nums1.length == m + n
 * nums2.length == n
 * 0 <= m, n <= 200
 * 1 <= m + n <= 200
 * -10^9 <= nums1[i], nums2[j] <= 10^9
 *
 *
 *
 * Follow up: Can you come up with an algorithm that runs in O(m + n) time?
 *
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i1: usize = 0;
        let mut i2: usize = 0;
        let mut temp_nums = std::collections::VecDeque::new();

        while i1 < (m + n) as usize {
            let mut v1 = nums1[i1];
            // make v1 huge after considering all of num1's m items
            if i1 >= m as usize {
                v1 = 1000000001 // 10^9 +1, might as well be infinity
            }
            match (nums2.get(i2), temp_nums.front()) {
                (Some(v2), Some(vt)) => {
                    let vt = *vt;
                    let v2 = *v2;
                    if vt <= v1 && vt <= v2 {
                        // don't add infinity to temp_nums
                        if v1 != 1000000001 {
                            temp_nums.push_back(nums1[i1]);
                        }
                        nums1[i1] = temp_nums.pop_front().unwrap();
                    } else if v2 <= v1 && v2 <= vt {
                        if v1 != 1000000001 {
                            temp_nums.push_back(nums1[i1]);
                        }
                        nums1[i1] = v2;
                        i2 += 1;
                    }
                    // else if v1 <= v2 && v1 <= vt {}
                    i1 += 1;
                }
                // temp_nums is empty
                (Some(v2), None) => {
                    if v1 > *v2 {
                        if v1 != 1000000001 {
                            temp_nums.push_back(nums1[i1]);
                        }
                        nums1[i1] = *v2;
                        i2 += 1;
                    }
                    i1 += 1;
                }
                // nums2 is exhausted
                (None, Some(vt)) => {
                    if v1 > *vt {
                        if v1 != 1000000001 {
                            temp_nums.push_back(nums1[i1]);
                        }
                        nums1[i1] = temp_nums.pop_front().unwrap()
                    }
                    i1 += 1;
                }
                // should happen only when nums2 starts empty.
                (None, None) => i1 += 1,
            }
        }
    }
}
// @lc code=end
