#![allow(dead_code)]
pub struct Solution;

//mod p26_remove_duplicates_from_sorted_array;
mod p1491_average_salary_excluding_the_minimum_and_maximum_salary;
mod p169_majority_element;
mod p189_rotate_array;
mod p27_remove_element;
mod p80_remove_duplicates_from_sorted_array_ii;
mod p88_merge_sorted_array;

fn main() {
    // test solutions like so:
    let mut nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let val = Solution::rotate(&mut nums, 3);
    println!("{:?}", nums);
}
