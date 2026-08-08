use std::collections::HashMap;


pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut map:HashMap<i32, i32> = HashMap::with_capacity(nums.len() + 1);
    
    map.insert(0 ,1);
    
    let mut count = 0;
    let mut sum = 0;
    for i in 0..n {
       sum += nums[i];

       if let Some(value) = map.get(&(sum - k)) {
           count += value;
       }
       *map.entry(sum).or_insert(0) += 1;
    }
    count
}


fn main() {
    let nums = vec![1,1,1];
    let ans = subarray_sum(nums, 2);

    println!("count: {}", ans);
}