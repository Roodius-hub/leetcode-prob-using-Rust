use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut map:HashMap<i32, i32> =  HashMap::new();
    map.insert(0, 1);

    let mut current_sum = 0;
    let mut count = 0;
    
    for val in nums {
        
        current_sum += val;

        let mut isHave = current_sum - k;
        if let Some(freq) = map.get(&isHave) {
            count += freq;
        }
        *map.entry(current_sum).or_insert(0) += 1;
    }

    count
}

fn main() {
    let nums = vec![1, 1, 1];
    let k = 2;
    let ans = subarray_sum(nums, k);

    println!("{}", ans);
}
