pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
    if nums.is_empty() {
        return vec![-1,-1];
    }
    let mut result:Vec<i32> =  vec![];
    // let mut ans:Vec<i32> = vec![];

    
    let mut left:i32 = 0;
    let mut right:i32  = (nums.len() -1) as i32;

    while left <= right {
        let  mid = left + (right - left) /2;
        let mid_idx = mid as usize;
        if nums[mid_idx] == target {
            result.push(mid as i32);
            right = mid - 1;
        } else if nums[mid_idx] < target {
            left = mid + 1;
        } else {
            right = mid -1;
        }
    }

    if result.is_empty() {
           return vec![-1, -1];
    }

    let mut left:usize = 0;
    let mut right:usize = nums.len() -1;
    
    while left <= right {
        let mid = left + (right - left ) /2 as usize ;
        if nums[mid] == target {
            result.push(mid as i32);
            left = mid + 1;
        } else if nums[mid] < target {
            left  = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result.sort();

    vec![result[0], result[result.len() -1]]
}


fn main() {
    let  nums = vec![5,7,7,8,8,10];
    let  target = 8;
    let ans = search_range(nums, target);
    println!("{:?}", ans);
}