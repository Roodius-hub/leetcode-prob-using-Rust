
//Sorting approach
// pub fn is_anagram(s:String,t:String) -> bool {
//     let mut new_s:Vec<char> = s.chars().collect();
//     let mut new_t:Vec<char> = t.chars().collect();

//     new_s.sort_unstable();
//     new_t.sort_unstable();

//     let sorted_s:String = new_s.into_iter().collect();
//     let sorted_t:String = new_t.into_iter().collect();

//     sorted_s == sorted_t
// }

use std::collections::HashMap;

pub fn is_anagram(s:String,t:String) -> bool {

    if s.len() != t.len() {
        return false
    }

    let mut map:HashMap<char, i32> = HashMap::new();

    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }

    for ch in t.chars() {
        match map.get_mut(&ch)  {
            Some(count) => {
                *count -= 1;
                if *count < 0 {
                    return false
                }
            }
            None => return false
        }
    }

    true
}

fn main() {
    let s = String::from("rat");
    let t = String::from("car");
    let ans = is_anagram(s,t);
    println!("Is Anagram: {}", ans);
}