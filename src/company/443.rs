use std::{collections::HashMap};


pub fn compress(chars: &mut Vec<char>) -> i32 {

    if chars.len() < 1 || chars.is_empty() {
        let new_s:String = chars.iter().collect();
        return new_s.len() as i32
    }

    let mut map:HashMap<char, i32> = HashMap::new();

    for ch in chars {
        *map.entry( *ch).or_insert(0) += 1;
    }

    println!("{:?}", map);
    let mut result:String = String::new();
    for (ch, v) in map.iter() {
        if let Some(value) = map.get(&ch) {
            println!("{}",value);
            result.push(*ch);
            result.push(char::from_digit(*value as u32, 10).unwrap());
        }
    }
    result.len() as i32
}


fn main() {
    // let mut chs:Vec<char> = vec!['a','a','b','b','c','c','c'];
    let mut  new_ch:Vec<char> = vec!['a'];
    let len = compress(&mut new_ch);
    println!("compressed vector length: {}", len);
}