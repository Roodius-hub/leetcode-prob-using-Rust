


pub fn  find_pairs(nums:Vec<i32>, k:i32) -> i32 {
    let mut count = 0;
    let n = nums.len();

    for i in 0..n {
        for j in i+1..n {
            if nums[i] - nums[j] == k {
                count +=1;
            }
        }
    }

    count

}




fn main() {
    let nums = vec![3, 1, 4, 1, 5];
    let ans = find_pairs(nums, 2);

    println!("{}" ,ans);

}