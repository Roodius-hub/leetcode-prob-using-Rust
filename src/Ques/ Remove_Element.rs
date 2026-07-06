
pub fn remove_elements(nums: &mut Vec<i32>, val:i32) -> i32 {    

        let mut k = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k +=1;
            }
        }
        nums.truncate(k);
        k as i32
}   


fn main() {
    let mut nums = vec![3,2,2,3];
    let ans = remove_elements(&mut nums, 3);
    print!("{}", ans);
}   