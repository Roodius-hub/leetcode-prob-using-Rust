use std::{collections::HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map:HashMap<i32,i32> = HashMap::new();

       // loop 
    for n in nums {
        *freq_map.entry(n).or_insert(0) += 1;
    }
    
    // into vector pair  values  
    let mut freq_vec:Vec<(i32,i32)> = freq_map.into_iter().collect();
    
    // sort 
    freq_vec.sort_by(|a,b| b.1.cmp(&a.1));

    // take first k elements 
    freq_vec.into_iter().take(k as usize).map(|(num, _freq)| num).collect()
}

fn main(){
    let nums = vec![1,2,1,1,2,1,3];
    let k = 2;
    let result  = top_k_frequent(nums.clone(), k);  
    
    println!("{:?}", result);
}