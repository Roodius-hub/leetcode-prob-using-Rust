
use std::collections::HashMap;

pub fn majority_elements(nums:Vec<i32>) -> i32 {
    let mut  map:HashMap<i32, i32> = HashMap::new();
    let  n = nums.len();

    for num in nums {
        let  count = map.entry(num).or_insert(num);
        *count += 1;

        if *count > (n / 2) as i32 {
            return num;
        }

    }
    -1
}


fn main() {
    let nums:Vec<i32>  = vec![3,2,3];
    let ans = majority_elements(nums);
    println!("element: {}", ans);
}