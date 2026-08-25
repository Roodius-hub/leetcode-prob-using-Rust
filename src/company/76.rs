use std::collections::HashMap;


// pub fn is_right(mapt:&HashMap<char, i32>, maps:&HashMap<char, i32>) -> String {

// }

pub fn min_window(s:String, t:String) -> String {
    let mut need:HashMap<char, i32>  = HashMap::with_capacity(256);


    for ch in  t.chars() {
        *need.entry(ch).or_insert(0) += 1;
    }

    let mut window:HashMap<char,i32> = HashMap::with_capacity(256);
    let chars:Vec<char> = s.chars().collect();

    let mut left = 0;
    let mut count = 0;
    let required = need.len();

    let mut best_start = 0;
    let mut best_len = std::usize::MAX;

    for right in 0..chars.len() {
        let ch = chars[right];

        if need.contains_key(&ch) {
            *window.entry(ch).or_insert(0) += 1;

            if window[&ch] == need[&ch] 
            {
                count += 1;
            }

        }

        while count == required {
            let len = right - left + 1;

            if len < best_len {
                best_len = len;
                best_start = left;
            }

            let left_char = chars[left];

            if need.contains_key(&left_char) {
                if window[&left_char] == need[&left_char] {
                    count -= 1;
                }
                *window.get_mut(&left_char).unwrap() -= 1;
            }
            left += 1;
        }
    }
        
    let ans:String = chars[best_start..best_start + best_len].iter().collect();

    if best_len == usize::MAX {
    return String::new();
} else {
    ans
}
    
}

fn main() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");

    let ans = min_window(s, t);

    println!("{}", ans);

}
