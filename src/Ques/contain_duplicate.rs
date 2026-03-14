use std::collections::HashSet;

pub fn contain_duplicate(nums:Vec<i32>) -> bool {
    let mut hashset = HashSet::<i32>::new();

    for i  in nums {
        if !hashset.contains(&i){
            hashset.insert(i);
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