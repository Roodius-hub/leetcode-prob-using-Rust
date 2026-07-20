
fn max_profit(nums: Vec<i32>) -> i32 {
    let mut max_prof = 0;
    let mut min_price = std::i32::MAX;

    for i in 0..nums.len() {
        min_price = std::cmp::min(min_price, nums[i]);
        max_prof = std::cmp::max(max_prof, nums[i] - min_price);
    } 
    max_prof
}

fn main() {
    let nums = vec![7,1,5,3,6,4];
    let ans = max_profit(nums);
    println!("Max Profit: {}", ans);
}