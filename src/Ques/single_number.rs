/*
Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
You must implement a solution with a linear runtime complexity and use only constant extra space.
*/
use std::{collections::HashSet, vec};
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
    
pub fn single_number_using_xor(nums:Vec<i32>) -> i32 {
    let mut unique_number:i32 = 0;
    for num in nums {
        unique_number ^= num;
    }
    return unique_number
}

fn main(){
    let nums:Vec<i32> = vec![2,2,1];
    let ans = single_number(nums.clone());
    println!(" nums is: {}", ans);

    let mut nums2:Vec<i32> = vec![];
    nums2 = nums.clone();
    let ans2 = single_number_using_xor(nums2);
    println!("xor: {}", ans2);
}