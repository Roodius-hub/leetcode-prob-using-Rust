

pub fn str_str(haystack: String, needle: String) -> i32 {
    
    let n = haystack.len();
    let m = needle.len();
    let mut ans:i32 = -1;
    for i in 0..(n-m) {
        if haystack[i..i + m] == needle {
            ans = i as i32;
            break;
        }
    }
    ans
}   
 

fn main() {
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");
    let ans = str_str(haystack, needle);
    println!("index: {}", ans);
}