/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
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
