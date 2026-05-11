use std::{collections::HashMap, vec};

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
    let digits = digits.bytes();
    
    if let Some(letters) = keypad.get(&'2') {
        for ch in letters.chars() {
            
        }
    }
    
    vec![]
}

fn main() { 
    
}