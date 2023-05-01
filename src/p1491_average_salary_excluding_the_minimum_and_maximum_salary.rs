/*
 * @lc app=leetcode id=1491 lang=rust
 *
 * [1491] Average Salary Excluding the Minimum and Maximum Salary
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
