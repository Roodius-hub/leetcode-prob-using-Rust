
// use std::collections::HashMap;
use std::collections::HashSet;


// pub fn find_duplicate(nums: Vec<i32>) -> i32 {

//     if nums.len() == 1 { 
//         return nums[0]
//     }
    
//     let mut map:HashMap<i32, i32> = HashMap::new();
//     let mut ans = 0;
//     for i in nums {
//         let count = map.entry(i).or_insert(0);
//         *count +=1;

//         if *count > 1  {
//             ans = i;
//         }
//     }
//     ans
// }

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut seen = HashSet::with_capacity(nums.len());
    let mut ans = 0;
    for num in nums {
            if !seen.insert(num) { // insert returns false if already present
                ans = num;
            }
        }
        ans
}



fn main() {
     let nums = vec![3,1,3,4,2];
     let ans = find_duplicate(nums);
     println!("Duplicates: {}", ans);
}