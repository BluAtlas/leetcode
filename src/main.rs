#![allow(dead_code)]
pub struct Solution;

//mod p26_remove_duplicates_from_sorted_array;
mod p1491_average_salary_excluding_the_minimum_and_maximum_salary;
mod p27_remove_element;
mod p80_remove_duplicates_from_sorted_array_ii;
mod p88_merge_sorted_array;

fn main() {
    // test solutions like so:
    let nums = vec![1000, 2000, 3000];
    let val = Solution::average(nums);
    println!("{}", val)
}
