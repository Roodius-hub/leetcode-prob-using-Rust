use std::collections::HashMap;


pub fn min_window(s:String, t:String) -> String {
    let mut mapt:HashMap<char, i32>  = HashMap::with_capacity(256);
    let mut maps:HashMap<char,i32> = HashMap::with_capacity(256);
    let new_s:Vec<char> = s.chars().collect();

    let ans:String = String::new();
    let mut left = 0;
    let mut count =  0;
    let mut st_index = 0;

    for ch in  t.chars() {
        *mapt.entry(ch).or_insert(0) += 1;
    }

    for right in 0..new_s.len() {
        *mapt.entry(new_s[right]).or_insert(0) += 1;

        while right < new_s.len() {
            
        }   

    }
        

    ans
}

fn main() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
}
