
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    
    let n = nums.len();
    let mut ans:Vec<i32> = vec![1;n];

    let mut left = 1;
    
    for i in 0..n {
        ans[i] = left;
        left *= nums[i];
    }

    let mut right = 1;

    for i in (0..n).rev() {
        ans[i] *= right;
        right *= nums[i];
    }
    ans
}


fn main() {
    let nums = vec![1,2,3,4];
    let  ans =  product_except_self(nums);    
    println!("{:?}", ans);
}