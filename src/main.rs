#![allow(dead_code)]
pub struct Solution;

mod p26_remove_duplicates_from_sorted_array;
mod p27_remove_element;
mod p88_merge_sorted_array;

fn main() {
    // test solutions like so:
    let mut nums = vec![1, 2];
    let val = Solution::remove_duplicates(&mut nums);
    println!("{:?}, {}", nums, val)
}
