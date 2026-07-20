

// two sum

pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut  ans:Vec<i32> = Vec::new();
    
    let mut indexed: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v,i)).collect();

    while let Some((last_val, last_idx)) = indexed.pop() {
    
        for (i, &(val, idx)) in indexed.iter().enumerate() {
            if val + last_val == target {
                ans.push(i as i32);
                ans.push(last_idx as i32);
                return ans;
            }
        }
    }
    ans
}



fn main() {
    let  nums = vec![2,7,11,15];
    let target = 9;
    let ans = two_sum(nums, target);
    println!("{:?}", ans);
}