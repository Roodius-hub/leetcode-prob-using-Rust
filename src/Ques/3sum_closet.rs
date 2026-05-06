/*
Given an integer array nums of length n and an integer target, find three integers at distinct indices in nums such that the sum is closest to target.

Return the sum of the three integers.

You may assume that each input would have exactly one solution.

 

Example 1:

Input: nums = [-1,2,1,-4], target = 1
Output: 2
Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
Example 2:

Input: nums = [0,0,0], target = 1
Output: 0
Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).

*/

use std::vec;

pub fn three_sum_closet(mut nums: Vec<i32>, target: i32) -> i32 {
    // sorting 
    nums.sort();    
    let n:usize = nums.len();
    let mut result:Vec<i32> = vec![];

    let mut currentSum = 0;
    for i in 0..nums.len() {
        let mut j:usize = i + 1;
        let mut k:usize = n - 1;

        while j < k {
            currentSum = nums[i] + nums[j] + nums [k];

            // if fist three  number contain target value
            if currentSum == target {
                return currentSum    
            }

            

        }

    }


    1
} 

fn main(){
    let nums = vec![-1,2,1,-4];
    three_sum_closet(nums, 1);

}