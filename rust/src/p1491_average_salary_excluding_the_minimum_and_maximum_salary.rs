/*
 * @lc app=leetcode id=1491 lang=rust
 *
 * [1491] Average Salary Excluding the Minimum and Maximum Salary
 *
 * https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/description/
 *
 * algorithms
 * Easy (61.06%)
 * Likes:    1308
 * Dislikes: 144
 * Total Accepted:    216.8K
 * Total Submissions: 351.7K
 * Testcase Example:  '[4000,3000,1000,2000]'
 *
 * You are given an array of unique integers salary where salary[i] is the
 * salary of the i^th employee.
 *
 * Return the average salary of employees excluding the minimum and maximum
 * salary. Answers within 10^-5 of the actual answer will be accepted.
 *
 *
 * Example 1:
 *
 *
 * Input: salary = [4000,3000,1000,2000]
 * Output: 2500.00000
 * Explanation: Minimum salary and maximum salary are 1000 and 4000
 * respectively.
 * Average salary excluding minimum and maximum salary is (2000+3000) / 2 =
 * 2500
 *
 *
 * Example 2:
 *
 *
 * Input: salary = [1000,2000,3000]
 * Output: 2000.00000
 * Explanation: Minimum salary and maximum salary are 1000 and 3000
 * respectively.
 * Average salary excluding minimum and maximum salary is (2000) / 1 = 2000
 *
 *
 *
 * Constraints:
 *
 *
 * 3 <= salary.length <= 100
 * 1000 <= salary[i] <= 10^6
 * All the integers of salary are unique.
 *
 *
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut largest = 2;
        let mut smallest = 1000001;
        let mut total = 0;

        for i in salary.iter() {
            if *i < smallest {
                smallest = *i;
            }
            if *i > largest {
                largest = *i;
            }
            total += *i;
        }

        return (total - largest - smallest) as f64 / (salary.len() - 2) as f64;
    }
}
// @lc code=end
