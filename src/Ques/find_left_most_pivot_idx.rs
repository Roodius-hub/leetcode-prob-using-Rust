
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    // validate that  nums is array of integer only 
    println!("{:?}", nums);

    let mut left_sum:i32 = 0;
    let mut right_sum:i32  = nums.iter().fold(0,|a,b| a + b);
    println!("sum: {}", right_sum);
     
    for i in 0..nums.len() {
         right_sum -= nums[i];
        if left_sum == right_sum {
            return i as i32;
        }
        left_sum += nums[i];

    }

    -1
}

fn main(){
    let nums:Vec<i32> = vec![1,7,3,6,5,6];
    let ans = pivot_index(nums);
    println!("pivot is: {}", ans);
}