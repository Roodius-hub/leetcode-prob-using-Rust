// Input: [1,8,6,2,5,4,8,3,7]
// Output: 49

// brute force approach -> it will give us TLE
// pub fn max_area(height: Vec<i32>) -> i32 {
//     let mut max_water = 0;
//     let n = height.len();
    
//     for i in 0..n {
//         for j in i+1..n {
//             let width  = j - i; 
//             let height = std::cmp::min(height[i], height[j]);
//             let area = (width as i32) * (height as i32);
//             if area > max_water {
//                 max_water = area;
//             }
//         }
//     }

//     max_water
// }

// optimal 
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = height.len();
    let mut l = 0;
    let mut r = n-1;
    while l < r {
        let wt = (r as i32) - (l as i32);
        let ht = std::cmp::min(height[l], height[r]);
        let area = (wt as i32) * (ht as i32);
        ans = std::cmp::max(ans, area);

        if height[l] < height[r] { 
            l += 1;
        } else {
            r -= 1;
        }
    }
    
    ans
}

fn main() {
    let height = vec![1,8,6,2,5,4,8,3,7];

    let ans = max_area(height);
    println!("answer: {}", ans);
}