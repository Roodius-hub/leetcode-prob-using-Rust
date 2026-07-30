pub fn next_permutation(nums:&mut Vec<i32>) {
    if nums.len() < 2 {
        println!("Length is less then 2");
        return;
    }   

    let n = nums.len();
    
    let mut pivot:Option<usize> = None;

    for i in (0..n -1).rev() {
        if nums[i] < nums[i+1] {
            pivot = Some(i);
            break;
        }
    }
    if pivot.is_none() {
        println!("Pivot Not found");
        nums.reverse();
        return;
    }
    
    let pivot_index = pivot.unwrap();

    for i in (pivot_index+1..n).rev() {
        if nums[i] > nums[pivot_index] {
            nums.swap(pivot_index,i);
            break;
        }
    }
    nums[pivot_index+1..].reverse()
}


fn main() {
    let mut nums = vec![1,2,3];
    next_permutation(&mut nums);

    println!("{:#?}", nums);
        
}