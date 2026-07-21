// contains duplicates
use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashMap::new(); 

    for val in nums {
        let count = map.entry(val).or_insert(0);
        *count += 1;
        if *count > 1{
            return true;
        }
    }
    false
}   

fn main() {
    let nums = vec![1,2,3,1];
    let ans  = contains_duplicate(nums);
    println!("{}", ans);
}