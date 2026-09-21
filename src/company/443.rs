

pub fn compress(chars: &mut Vec<char>) -> i32 { 

    let mut i = 0; 
    let mut write = 0;
    let n = chars.len();

    while i < n {
        let temp = chars[i];
        let mut count = 0;

        while i < n && temp == chars[i] {
            count += 1;
            i += 1;
        }

        chars[write] = temp;
        write +=1;

        if  count >= 2 {
            for ch in count.to_string().chars() {
                chars[write] = ch;
                write += 1;
            }
        }        
    }

    chars.truncate(write);
    chars.len() as i32
}


fn main() {
    // let mut chs:Vec<char> = vec!['a','a','b','b','c','c','c'];
    let mut  new_ch:Vec<char> = vec!['a', 'a', 'b', 'b', 'c', 'c','c'];
    let len = compress(&mut new_ch);
    println!("compressed vector length: {}", len);
}