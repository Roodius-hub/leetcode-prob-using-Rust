/*
Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
You must implement a solution with a linear runtime complexity and use only constant extra space.
*/
use std::collections::HashSet;
pub fn single_number(nums:Vec<i32>) -> i32 {
    let mut set:HashSet<i32> = HashSet::<i32>::new();

    for i in 0..nums.len() {
        if !set.contains(&nums[i]) {
            set.insert(nums[i]);
        } else {
            set.remove(&nums[i]);
        }
    }
    *set.iter().next().unwrap()
}
    

fn main(){
    let nums:Vec<i32> = vec![2,2,1];
    let ans = single_number(nums);
    println!(" nums is: {}", ans);
}