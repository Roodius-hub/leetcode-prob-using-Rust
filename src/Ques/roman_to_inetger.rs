use std::collections::HashMap;

pub fn roman_to_int( s : String) -> i32 {
    let new_map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let  ch:Vec<char> = s.chars().collect();
    let mut result = 0;

    for i in 0..s.len() {
        let curr = new_map[&ch[i]];

        if i + 1 < ch.len() && curr < new_map[&ch[i + 1]] {
            result -= curr;
        } else {
            result += curr;
        }
    }
    result
}   

fn main() {
    let roman_string = String::from("LVIII");
    let ans = roman_to_int(roman_string);
    println!("{}", ans)
}