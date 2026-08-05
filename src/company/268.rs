

pub fn missing_number(mut nums:Vec<i32>) -> i32 {
    let n = nums.len();
    
    let total_sum = n * (n+1)/2;
    println!("total: {}",total_sum);
        
    let mut current_sum = 0;

    for i in nums {
        current_sum += i;
    }
    
    total_sum as i32- current_sum as i32
}


fn main() {
    let nums = vec![3,0,1];
    let ans = missing_number(nums);
    println!("Missing Number: {}", ans);
}