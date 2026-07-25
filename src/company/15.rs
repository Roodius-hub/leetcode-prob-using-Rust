
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans:Vec<Vec<i32>> = Vec::new();
    let n = nums.len();

    if n < 3 {
        return ans;
    }
    
    nums.sort();

    println!("{:?}", nums);

    for first in 0..n-2 {

        if first > 0 && nums[first] == nums[first -1] {
            continue;
        }

        let mut  left = first + 1;
        let mut right = n -1;

        while left <  right {
            let current_sum = nums[first] + nums[left] + nums[right];
            
            if current_sum < 0 {
                left += 1;
            } else if current_sum > 0 {
                right -= 1;
            } else {
                ans.push(vec![nums[first], nums[left], nums[right]]);

                left += 1;
                right -= 1;

                while left < right && nums[left] == nums[left -1] {
                    left += 1;
                }

                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            }
        }   
    }
    
    ans
}


fn main() {
    let nums = vec![-1,0,1,2,-1,-4];
    let ans = three_sum(nums);
    print!("{:#?}", ans);
        
}