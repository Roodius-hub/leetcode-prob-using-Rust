
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n == 0 { return vec![]; }
    let mut preffix = vec![1; n];
    let mut suffix = vec![1; n];
    let mut ans = vec![0; n];

    //  preffix
    for i in 1..n {
        preffix[i] = preffix[i -1] * nums[i-1];
    }
    println!("Preffix: {:?}", preffix);

    //suffix 
    for i in (0..n -1).rev() {
        suffix[i] = suffix[i+1] * nums[i+1];
    }

    println!("Suffix: {:?}", suffix);

    for i in 0..n {
        ans[i] = suffix[i] * preffix[i];
    }

    ans
}

fn main() {
    let nums = vec![1,2,3,4];
    let ans = product_except_self(nums);
    println!("Final answer: {:?}", ans);
}