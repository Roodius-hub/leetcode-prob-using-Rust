use std::collections::HashSet;

pub fn contain_duplicate(nums:Vec<i32>) -> bool {
    let mut hashset = HashSet::<i32>::new();

    for i  in 0..nums.len() {
        if !hashset.contains(&nums[i]){
            hashset.insert(nums[i]);
        } else {
            return true
        }
    }

    false
}


fn main(){
    let nums:Vec<i32> = vec![1,1,1,3,3,4,3,2,4,2];
    let ans = contain_duplicate(nums);
    println!("{}",ans);
}