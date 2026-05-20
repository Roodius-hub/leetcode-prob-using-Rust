use std::cmp::max;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut currentSum = 0;
    let mut max_sum = std::i32::MIN;

    for val in nums {
        currentSum  += val;
        max_sum = max(currentSum, max_sum);
        if currentSum < 0 {
            currentSum = 0;
        }
    }
    max_sum
}

fn main() {
    let nums:Vec<i32> = vec![-2,1,-3,4,-1,2,1,-5,4];
    
    let ans = max_sub_array(nums);
    println!("{}", ans);
}