#![allow(dead_code)]
pub struct Solution;

//mod p26_remove_duplicates_from_sorted_array;
//mod p121_best_time_to_buy_and_sell_stock;
mod p122_best_time_to_buy_and_sell_stock_ii;
mod p1491_average_salary_excluding_the_minimum_and_maximum_salary;
mod p169_majority_element;
mod p189_rotate_array;
mod p27_remove_element;
mod p45_jump_game_ii;
mod p55_jump_game;
mod p80_remove_duplicates_from_sorted_array_ii;
mod p88_merge_sorted_array;

fn main() {
    // test solutions like so:
    //let nums = vec![2, 3, 1, 1, 4];
    //let nums = vec![1, 1, 0, 1];
    //let nums = vec![2, 0, 0];
    //let nums = vec![1, 2, 0, 1, 4, 0, 0, 0, 1, 6];
    let nums = vec![2, 3, 1];
    //let nums = vec![2, 3, 0, 1, 4];

    let val = Solution::jump(nums);
    println!("{:?}", val);
}
