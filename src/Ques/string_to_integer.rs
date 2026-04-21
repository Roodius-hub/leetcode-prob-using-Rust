
pub fn my_atoi(s: String) -> i32 {
    let byte_s = s.as_bytes();
    let mut i:usize = 0;
    let  length = s.len();

    // skip white spaces 
    while i < length && byte_s[i] == b' ' {
        i += 1;
    }

    // if string contain only spaces
    if i == length { return 0; }

    //checking signed character 
    let mut sign = 1;
    if byte_s[i] == b'-' {
        sign = -1;
    }

    // move past if - , + 
    if byte_s[i] == b'-' || byte_s[i] == b'+' {
        i += 1;
    }

    let mut res = 0;
    let overflowFlag = i32::MAX / 10;


    while i < length {
        if !byte_s[i].is_ascii_digit() {
            break;
        }

        let mut digit = (byte_s[i] - b'0') as i32;

        if res > overflowFlag || res == overflowFlag && digit > 7 {
            let ans = if sign == 1 {i32::MAX} else {i32::MIN};
            return ans
        } else {
            res = (res * 10) + digit;
            i += 1;
        }
    }
    res * sign
}




fn main() {
    let my_string = String::from(" -42");

   let ans =  my_atoi(my_string);

   println!("ans: {}", ans);
}