


// pub fn max_product(nums:Vec<i32>) -> i32 {
//     let n = nums.len();
//     let mut current_prod = nums[0];
//     let mut max_prod:i32 = nums[0];

//     for i in 1..n {
//         let combined_prod = current_prod * nums[i];

//         if nums[i] > combined_prod {
//             current_prod = nums[i]
//         } else {
//             current_prod = combined_prod
//         }

//         if current_prod > max_prod {
//             max_prod = current_prod
//         }
        
//     }
    
//     max_prod
// }

pub fn max_product(nums:Vec<i32>) -> i32 {
    let n = nums.len();
    let mut max_prod = nums[0];
    let mut max_ending = nums[0];
    let mut min_ending = nums[0];

    for i in 1..n {
        let num = nums[i];
        let temp_max = max_ending;  

        max_ending = std::cmp::max(
            num, std::cmp::max(max_ending * num, min_ending * num)
        );

        min_ending = std::cmp::min(
            num, std::cmp::min(temp_max * num, min_ending * num)
        );

        max_prod = std::cmp::max(max_prod, max_ending);
        
    }
    max_prod
}

fn main() {
    let nums = vec![-2,3,-4];
    let ans = max_product(nums);

    println!("max-product: {}", ans);
}