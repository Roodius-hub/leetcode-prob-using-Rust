

// pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//        let n = nums.len();
//        let mut max_sum = std::i32::MIN;

//        for i in 0..n {
//            for j in i..n {
            
//                let mut current_sum = 0;
               
//                for k in i..=j {
//                    current_sum += nums[k];
//                }

//                if current_sum > max_sum {
//                    max_sum = current_sum
//                }
//            }
//        }
//        max_sum
// }

// pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//     let n  = nums.len();
//     // print!("{}", n);
//     let mut max_sum = nums[0];

//     for i in 0..n {
//         let mut current_sum = 0;
//         for j in i..n {
//             current_sum += nums[j];

//             if current_sum > max_sum {
//                 max_sum = current_sum;
//             }
//         }
//     }
    
//     max_sum
// }

// kadans
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let n  = nums.len();
    // print!("{}", n);
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for i in 1..n {
        let combined_sum = current_sum + nums[i];

        if nums[i] > combined_sum {
            current_sum = nums[i];
        } else {
            current_sum = combined_sum;
        }
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }
    
    max_sum
}


fn main() {
    let nums = vec![5,4,-1,7,8];
    let ans = max_sub_array(nums);
    println!("{}", ans);
}