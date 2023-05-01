#![allow(dead_code)]
pub struct Solution;

//mod p26_remove_duplicates_from_sorted_array;
mod p27_remove_element;
mod p80_remove_duplicates_from_sorted_array_ii;
mod p88_merge_sorted_array;

fn main() {
    // test solutions like so:
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let val = Solution::remove_duplicates(&mut nums);
    println!("{:?}, {}", nums, val)
}
