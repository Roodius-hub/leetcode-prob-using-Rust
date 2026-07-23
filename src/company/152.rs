


pub fn max_product(nums:Vec<i32>) -> i32 {
    let n = nums.len();
    let mut current_prod = nums[0];
    let mut max_prod:i32 = nums[0];

    for i in 1..n {
        let combined_prod = current_prod * nums[i];

        if nums[i] > combined_prod {
            current_prod = nums[i]
        } else {
            current_prod = combined_prod
        }

        if current_prod > max_prod {
            max_prod = current_prod
        }
        
    }
    
    max_prod
}


fn main() {
    let nums = vec![-2,0,-1];
    let ans = max_product(nums);

    println!("max-product: {}", ans);
}