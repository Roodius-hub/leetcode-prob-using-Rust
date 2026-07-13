use std::{collections::HashMap, hash::Hash};

fn generate_permutation(s:String,words:Vec<String>) -> Vec<i32> {
    let mut freq:HashMap<String, i32> = HashMap::new();
    let mut curr:HashMap<String, i32> = HashMap::new();

    
    let n = s.len();
    let m = words.len();
    let wordsize = words[0].len();
    let windowsize = wordsize * m;


    for word in words {
        *freq.entry(word).or_insert(0) += 1;
    }

    


    vec![1,2]
}


fn main() {
    let words = vec![""];
    let s = String::from("");
}