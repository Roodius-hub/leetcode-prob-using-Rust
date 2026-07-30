

pub fn first_missing_positive(nums:Vec<i32>) -> i32 {
    let n = nums.len();

    for i in 0..n {
        while 1 >= nums[i] && nums[i] <= n as i32 {
            let mut j = nums[i] - 1;
            nums.swap(i, j as usi);
        }
    }
    1
}   

fn main() {
    let nums = vec![1,2,0];
    let ans = first_missing_positive(nums);
    println!("First missing positive: {}", ans);
}