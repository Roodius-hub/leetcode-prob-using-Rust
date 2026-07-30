


// bruteforce approach  -> we can take here  vector for both or single variable
// pub fn trap(height: Vec<i32>) -> i32 {
//     let mut ans = 0; 
//     let n = height.len();
    
//     for i in 0..n {

//         //left_max 
//         let mut left_max = 0;
//         for j in (0..=i).rev() {
//             left_max = std::cmp::max(left_max, height[j]) 
//         }
//         let mut right_max = 0;
//         for j in i..n {
//             right_max = std::cmp::max(right_max, height[j]);
//         }

//         ans += std::cmp::min(left_max, right_max) - height[i];

//     }
//     ans
// }

// two pointer approach 
pub fn trap(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = height.len();
    
    let mut l = 0;
    let mut  r = n-1; 

    let mut left_max = 0; 
    let mut right_max = 0;
    
    while l < r {
        left_max = std::cmp::max(left_max, height[l]);
        right_max = std::cmp::max(right_max, height[r]);

        if left_max < right_max {
            ans += left_max - height[l];
            l += 1;
        } else {
            ans += right_max - height[r];
            r -= 1;
        }
    }

    ans
}


fn main() {
  let  height = vec![4,2,0,3,2,5];
 let ans = trap(height);
 println!("{}", ans);
}