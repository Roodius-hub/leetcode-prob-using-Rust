/*
Given two strings s and t of lengths m and n respectively, return the minimum window substring of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".

The testcases will be generated such that the answer is unique.
*/
use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    let mut need: HashMap<char, i32> = HashMap::new();
    for ch in t.chars() {
        need.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut window:HashMap<char, i32> = HashMap::new();
    let left = 0;
    let matched:bool;
    

    "stuibg".to_string()        
}


fn main() {

}