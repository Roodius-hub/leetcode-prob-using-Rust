use std::{collections::HashMap, hash::Hash};


fn generate_permutation(s:String,words:Vec<String>) -> Vec<i32> {
    let mut freq:HashMap<String, i32> = HashMap::new();
    let mut curr:HashMap<String, i32> = HashMap::new();

    
    let n = s.len();
    let m = words.len();
    let wordsize = words[0].len();
    let windowsize = wordsize * m;
    let mut ans:Vec<i32> = vec![];
    // keep track frequncy
    for word in words {
        *freq.entry(word).or_insert(0) += 1;
    }

    for offset in 0..wordsize {
    let mut start = offset;

    while start + windowsize <= n {
        let mut curr = freq.clone();
        let mut matched = true;

        for j in 0..m {
            let left = start + j * wordsize;
            let right = left + wordsize;

            let currword = &s[left..right];

            if !curr.contains_key(currword) || curr[currword] == 0 {
                matched = false;
                break;
            }

            *curr.entry(currword.to_string()).or_insert(0) -= 1;
        }

        if matched {
            ans.push(start as i32);
        }

        start += wordsize;
    }
}
    ans
}


fn main() {
    let words = vec!["foo".to_string(),"bar".to_string()];
    let s = String::from("barfoothefoobarman");
    let ans = generate_permutation(s, words);

    for w in ans {
        println!("{}", w);
    }
}