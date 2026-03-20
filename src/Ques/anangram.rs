use std::{collections::HashMap};

pub fn anagram(s:String, t:String) -> bool {
    if s.len() != t.len() {
        return false
    }
    let mut map:HashMap<char, i32> = HashMap::new();  // map
    let splited_s:Vec<char> = s.chars().collect(); // chars 
    
    for c in splited_s {
        // *map.entry(c).or_insert(0) += 1;
        if map.contains_key(&c) {
            let value = map.get_mut(&c).unwrap();
            *value += 1;
        } else {
            map.insert(c , 1);
        }
    }

    for c in t.chars() {
        match map.get_mut(&c) {  // matching 
            Some(value) => {
                *value -= 1;
                if *value < 0 {
                    return false;
                }
            }
            None => return false
        }
    }
   
    true
}

fn main() {
    let s:String = String::from("anagram");
    let t: String = String::from("nagaram");
    let ans = anagram(s,t);
    println!("{}", ans);
}