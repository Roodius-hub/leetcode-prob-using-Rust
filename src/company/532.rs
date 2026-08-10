// brute force approach
// pub fn  find_pairs(nums:Vec<i32>, k:i32) -> i32 {
//     let mut count = 0;
//     let n = nums.len();

//     for i in 0..n {
//         for j in i+1..n {
//             if nums[i] != nums[j] && nums[i] - nums[j] == k {
//                 count +=1;
//             }
//         }
//     }
//     count
// }

// pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
//     if nums.len() < 2 || k < 0 {
//         return 0;
//     }

//     nums.sort();

//     let mut left = 0;
//     let mut right = 1;
//     let mut count = 0;

//     while right < nums.len() {
//         if left == right {
//             right += 1;
//             continue;
//         }

//         let diff = nums[right] - nums[left];

//         if diff < k {
//             right += 1;
//         } else if diff > k {
//             left += 1;
//         } else {
//             count += 1;

//             // Skip duplicate left values
//             let left_value = nums[left];
//             while left < nums.len() && nums[left] == left_value {
//                 left += 1;
//             }

//             // Skip duplicate right values
//             let right_value = nums[right];
//             while right < nums.len() && nums[right] == right_value {
//                 right += 1;
//             }
//         }
//     }

//     count
// }

use std::collections::{HashMap, HashSet};

pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {

    if k < 0 {
        return 0;
    }

    if k == 0 {
        let mut seen_new= HashMap::new();

        for num in nums {
            *seen_new.entry(num).or_insert(0) += 1;
        }
        let mut count = 0;

        for value in seen_new.values() {
            if * value >=2 {
                count += 1;
            }
        }
        return count;
    }
    
    let seen:HashSet<i32> = nums.iter().copied().collect();
    
    let mut count = 0;

    for &num in &seen {
        if seen.contains(&(num + k)) {
            count += 1;
        }
    }
    count
}

fn main() {
    let nums = vec![1,1,1,2,2];
    let ans = find_pairs(nums, 0);

    println!("{}", ans);
}
