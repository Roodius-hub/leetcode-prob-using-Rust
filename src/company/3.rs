
use std::{collections::HashMap, hash::Hash};



pub fn length_of_longest_substring(s: String) -> i32 {
    // longest substring
    let mut seen:HashMap<char, usize> = HashMap::new();
    let mut l = 0;
    let mut max_length = 0;

    for (r, ch) in s.chars().enumerate() {
        if let Some(&prev_idx) = seen.get(&ch) {
            l = l.max(prev_idx + 1);
        }
        seen.insert(ch, r);

        max_length = max_length.max(r-l+1);
    }   
    println!("map: {:?}", seen);
        
    max_length as i32
    
}


    

fn main() {
    let s = String::from("abcabcbb");
   let ans = length_of_longest_substring(s);
   println!("length_of_longest_substring: {}", ans);
}