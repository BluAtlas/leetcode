#![allow(dead_code)]
pub struct Solution;

mod p27_remove_element;
mod p88_merge_sorted_array;

fn main() {
    // test solutions like so:
    let mut nums = vec![3, 2, 2, 3];
    let val = Solution::remove_element(&mut nums, 3);
    println!("{:?}, {}", nums, val)
}
