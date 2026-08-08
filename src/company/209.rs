use std::collections::HashMap;

pub fn min_sub_array_len(target:i32, nums:Vec<i32>) -> i32 {

    let  mut min_window = std::i32::MAX;

    let mut right = 0;
    let mut left = 0;
    let mut current_sum = 0;
    
    while right < nums.len() {
        current_sum += nums[right];
        
        while current_sum >= target {
            let window_len = right - left + 1; // current window length
            
            min_window = std::cmp::min(min_window, window_len as i32); // comapring who's short

            current_sum -= nums[left]; // if sum greater shrink

            left +=1; // left go forward
            
        }
        right += 1;  // else right go  forward
    }

    if min_window == std::i32::MAX {
        0
    } else {
        min_window
    }
}



fn main() {
    let nums = vec![2,3,1,2,4,3];
    let ans = min_sub_array_len(7, nums);
    println!("min length: {}", ans);
}