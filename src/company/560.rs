use std::collections::HashMap;


pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut map:HashMap<i32, i32> = HashMap::new();
    
    map.insert(0 ,1);
    
    let mut count = 0;
    let mut sum = 0;
    for i in 0..n {
       sum += nums[i];

       if map.get(&(sum - k)).is_some() {
           count += 1;
       }

       *map.entry(i as i32).or_insert(0) += 1;
       
    }

    count
}


fn main() {
    
}