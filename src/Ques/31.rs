use std::mem::swap;

pub fn next_permutation(nums: &mut Vec<i32>) {

    if nums.len() < 3 {
        return;
    }
    
    let mut pivot = None;
    
        print!("{:?}", nums);
    
    for i in (0..nums.len() -1).rev() {
        if nums[i] < nums[i+1] {
            pivot = Some(i);
            break;
        }
    }

    if pivot.is_none() {
        nums.reverse();
        return;
    }
    
    print!("{:?}", nums);

    let pivot_idx = pivot.unwrap();
    
    for i in  (pivot_idx+1..nums.len()).rev() {
        if nums[i] > nums[pivot_idx]  {
            nums.swap(pivot_idx,i);
            break;
        }
    }
    nums[pivot_idx + 1..].reverse();
}


fn main() {
    let mut nums = vec![2,3,1];
    next_permutation(&mut nums);

    for val in nums {
        println!("{}", val);
    }
}