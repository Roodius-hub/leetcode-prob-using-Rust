
pub fn four_sum(mut nums: Vec<i64>, target: i64) -> Vec<Vec<i64>> {
    if nums.len() < 4 {
        return vec![];
    }
    
    nums.sort_unstable();
    let n = nums.len();
    let mut result = vec![];

    
    let mut x:i64 = 0;
    for i in 0..n-3 {
        if i > 0 && nums[i] == nums[i-1] {
            continue;
        }
        for j in (i+1)..(n-2) {
            
            let mut l = j + 1;
            let mut k = n-1;
            
            if j > i + 1 && nums[j] == nums[j-1] {
                continue;
            }            
            
            while l < k {
                x = nums[i] + nums[j] + nums[k] + nums[l];

                if x < target {
                    l += 1;
                } else if x > target {
                    k -= 1;
                } else {
                    println!("{}", x);
                    result.push(vec![nums[i], nums[j], nums[l], nums[k]]);
                    while l < k && nums[l] == nums[l + 1] { l += 1; }
                    while l < k && nums[k] == nums[k - 1] { k -= 1; }
                    l += 1;
                    k -= 1;
                }
            }   
        }       
    }

    result
}

fn main() {
    let nums = vec![1,0,-1,0,-2,2];
    
    let ans = four_sum(nums, 0);
    println!("vector: {:?}", ans);
}