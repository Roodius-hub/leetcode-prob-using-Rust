
// pub fn compress(chars: &mut Vec<char>) -> i32 {

//     if chars.is_empty() {
//         return 0;
//     }

//     let mut map:HashMap<char, i32> = HashMap::new();

//     for ch in chars {
//         *map.entry( *ch).or_insert(0) += 1;
//     }

//     println!("{:?}", map);
//     let mut result:String = String::new();
//     for (ch, v) in map.iter() {
//         if let Some(value) = map.get(&ch) {
//             println!("{}",value);
//             result.push(*ch);
//             if *value >= 2 {
//                 for ch in value.to_string().chars() {
//                     result.push(ch);
//                 }
//             }
//         }
//     }
//     println!("{}", result);
//     result.len() as i32
// }


pub fn compress(chars: &mut Vec<char>) -> i32 {

    let mut i = 0;
    
    let n = chars.len();
    let mut write = 0;
    while i < n {
        let ch = chars[i];
        let mut count = 0;

        while i < n && ch == chars[i] {
                count += 1;
                i += 1;
        }

        chars[write] = ch;
        write += 1;

        if count >= 2 {
            for ch in count.to_string().chars() {
                chars[write] = ch;
                write += 1;
            }
        }
    }    

        // Remove everything after the compressed part
        chars.truncate(write);
        write as i32
}

fn main() {
    // let mut chs:Vec<char> = vec!['a','a','b','b','c','c','c'];
    let mut  new_ch:Vec<char> = vec!['a', 'a', 'b', 'b', 'c', 'c','c'];
    let len = compress(&mut new_ch);
    println!("compressed vector length: {}", len);
}