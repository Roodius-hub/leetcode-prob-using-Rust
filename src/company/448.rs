use std::path::absolute;




pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = Vec::new();

    for i in 0..n {
        let index = nums[i].abs() as usize -1;
        if nums[index] > 0 {
            nums[index] -= nums[index]
        }
    }
    for i  in 0..n {
        if nums[i] > 0 {
            ans.push(i as i32 +1);
        }
    }
    // println!("{:?}", nums);
        
    ans
}


fn main() {
    let nums = vec![4,3,2,7,8,2,3,1];
    let ans = find_disappeared_numbers(nums);
    println!("{:?}",ans);
}