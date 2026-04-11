/*
    Given a string s, return the longest palindromic substring in s.
    
Example 1:

Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.
Example 2:

Input: s = "cbbd"
Output: "bb"

*/

//Function to expand around the center and find the length of the palindrome 
// And This is more basic method to check  string palindrom or not 
pub fn expand_from_center(my_string:&Vec<char>,mut _left:isize, mut _right:isize) -> usize {
    while _left >= 0 && _right < my_string.len() as isize && my_string[_left as usize] == my_string[_right as usize]{
        _left -= 1;
        _right += 1;
    }
    // return final length of any possible palindrom
    let length = _right - _left -1;
    length as usize
}

pub fn longest_palindromic_string(my_string:String) -> String {
    let into_v:Vec<char> = my_string.chars().collect();
    // println!("{:?}", into_v);

    let mut start:usize = 0;
    let mut end:usize = 0;

    for i in 0..into_v.len() {

        //when the center is a single character (odd)
        let len1 = expand_from_center(&into_v, i as isize, i as isize);

        // when the center is between two character
        let len2 = expand_from_center(&into_v, i as isize, i as isize + 1);

        // max lenth
        let final_len = std::cmp::max(len1, len2);

        // longest palindrom indexes
        if final_len > end - start {
            start = i - (final_len - 1)  / 2;
            end = i + (final_len / 2)
        }
    }
    into_v[start..=end].iter().collect()
}



fn main(){
    let new = String::from("babad");
    let longest_palindrome:String = longest_palindromic_string(new);
    println!("longest palindrome in a string: {}", longest_palindrome);
}