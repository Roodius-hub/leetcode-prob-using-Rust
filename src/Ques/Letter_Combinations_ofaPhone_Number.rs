use std::{collections::HashMap,vec};

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut keypad  = HashMap::new();

    keypad.insert('2', "abc");
    keypad.insert('3', "def");
    keypad.insert('4', "ghi");
    keypad.insert('5', "jkl");
    keypad.insert('6', "mno");
    keypad.insert('7', "pqrs");
    keypad.insert('8', "tuv");
    keypad.insert('9', "wxyz");

    
    let mut result = vec![];

    back_track(
        &digits,
        0,
        &keypad,
        String::new(),
        &mut result
    );

    
    result
}

pub fn back_track(
    digits:&str,
    index:usize,
    keypad:&HashMap<char,&str>, 
    current:String,
    result:&mut  Vec<String>
) {

    if index == digits.len() {
        result.push(current);
        return
    }

    // current
    let digit = digits.chars().nth(index).unwrap();

    if let Some(letters) = keypad.get(&digit) {
        for ch in letters.chars() {
            let mut next = current.clone(); // thats mean we are cloning a current character 
            next.push(ch);
            back_track(digits, index + 1, keypad, next, result);
        }
    }
}

fn main() { 
    let ans = letter_combinations("23".to_string());

    println!("{:?}", ans);    
}