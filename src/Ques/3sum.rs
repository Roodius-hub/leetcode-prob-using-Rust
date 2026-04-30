

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> { 
     nums.sort();
            

}
 

fn main() {
    let nums = vec![-1,0,1,2,-1,-4];

    let ans = three_sum(nums);
    println!("{}", ans);
}