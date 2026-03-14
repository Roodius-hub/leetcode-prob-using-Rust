
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut  combined_array_curr_idx = (m + n) as usize;
    let mut  num1_curr_idx: usize = m  as usize;
    let mut num2_curr_idx: usize = n as usize;

    while num2_curr_idx > 0 {
        if num1_curr_idx > 0 && nums1[num1_curr_idx - 1] > nums2[num2_curr_idx -1] {
              nums1[combined_array_curr_idx -1] = nums1[num1_curr_idx -1];
              num1_curr_idx -= 1;
        } else {
            nums1[combined_array_curr_idx -1] = nums2[num2_curr_idx -1];
            num2_curr_idx -= 1;
        } 
        combined_array_curr_idx -= 1;
    } 

}

fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];
    
    merge(&mut nums1, 3, &mut nums2, 3);

    for v in nums1 {
        print!(" {} ", v);
    }

}

