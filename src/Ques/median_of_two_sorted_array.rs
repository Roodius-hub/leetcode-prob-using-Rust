// /*Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

// The overall run time complexity should be O(log (m+n)).

 

// Example 1:

// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
// Example 2:

// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//  */

// pub fn median_of_two_sorted_array<>(mut _nums1:Vec<i32>, _nums2:Vec<i32> ) -> f32 {
    
//     for i in _nums2 {
//         _nums1.push(i);
//     }
//     println!("{:?}", _nums1);   

//     _nums1.sort();
//     let n = _nums1.len();

//     if n % 2 == 1 {
//        return  _nums1[n / 2] as f32;
//     } 

//     // if even
//     let mid1 = _nums1[(n/ 2) -1 ];
//     let mid2 = _nums1[n / 2];

//     (mid1 as f32 + mid2 as f32 ) / 2.0
// }


// fn main() {
//     let nums1 = vec![1,2];
//     let nums2 = vec![3,4];   
//     let ans = median_of_two_sorted_array(nums1, nums2);
//     println!("{}", ans);
// }


impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };
        let tot = a.len() + b.len();

        let mut lo_a: usize = 0;
        let mut hi_a: usize = a.len();
        loop {
            let mid_a = lo_a.midpoint(hi_a);
            let mid_b = (a.len() + b.len()).div_ceil(2) - mid_a;
            debug_assert!(mid_b <= b.len());
            let max_left_a = mid_a.checked_sub(1).map(|i| a[i]).unwrap_or(i32::MIN);
            let max_left_b = mid_b.checked_sub(1).map(|i| b[i]).unwrap_or(i32::MIN);
            let min_right_a = a.get(mid_a).copied().unwrap_or(i32::MAX);
            let min_right_b = b.get(mid_b).copied().unwrap_or(i32::MAX);
            if max_left_a <= min_right_b && max_left_b <= min_right_a {
                let max_left = f64::from(std::cmp::max(max_left_a, max_left_b));
                if tot.is_multiple_of(2) {
                    let min_right = f64::from(std::cmp::min(min_right_a, min_right_b));
                    break max_left.midpoint(min_right);
                } else {
                    break max_left;
                }
            } else if max_left_a > min_right_b {
                hi_a = mid_a - 1;
            } else {
                lo_a = mid_a + 1;
            }
            debug_assert!(lo_a <= hi_a);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn examples() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}