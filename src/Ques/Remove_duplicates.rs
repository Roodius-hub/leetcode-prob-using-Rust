use std::collections::HashMap;
use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // let mut map= HashSet::new();

    let mut k = 1;
    for i in 1..nums.len() 
    {
        if nums[i] != nums[k-1] {
            nums[k] = nums[i];
            k += 1;
        }
    }

    nums.truncate(k);

    k as i32
}   

fn main() {
    let mut  nums = vec![1,1,2];

     remove_duplicates(&mut nums);
    for num in nums{
        print!("{}, ", num);
    }
}