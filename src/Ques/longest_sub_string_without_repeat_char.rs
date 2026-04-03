/*
    Given a string s, find the length of the longest substring without duplicate characters.

            Example 1:

            Input: s = "abcabcbb"
            Output: 3
            Explanation: The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.
            Example 2:

            Input: s = "bbbbb"
            Output: 1
            Explanation: The answer is "b", with the length of 1.
            Example 3:

            Input: s = "pwwkew"
            Output: 3
            Explanation: The answer is "wke", with the length of 3.
            Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
*/
use std::cmp;

pub fn longest_sub_string_without_repeat_char(strr:String) -> i32 {
    
    if strr.len() <= 0 {
        return strr.len() as i32
    }

    let mut res:usize  = 0;

    let str_bytes = strr.as_bytes(); // convert in u8

    let mut left_pointer:usize = 0;
    // let mut right_pointer:usize = 0; we dont need it now 

    let mut vis:Vec<bool> = vec![false; 256];  // for all character

   for right_pointer in 0..str_bytes.len() {

        while vis[str_bytes[right_pointer] as usize] == true {

        vis[str_bytes[left_pointer] as usize] = false;

        left_pointer += 1  as usize;
    }

    vis[str_bytes[right_pointer] as usize] = true;

    res = cmp::max(res, right_pointer - left_pointer + 1);
    // right_pointer += 1 as usize;

    }

    res as i32
    
}

fn main() {
    let word = String::from("abcabcbb");

    let ans = longest_sub_string_without_repeat_char(word);
    println!("{}", ans);
}