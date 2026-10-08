

pub fn my_atoi(s: String) -> i32 {

    // skip - and " " places from string convert it intot  number
    // and rounding ineteger

    let into_byte:Vec<char> = s.chars().collect();
    let n = into_byte.len();
    let mut i = 0;
    // let mut result = 0;
    let flag = 2 * 31 -1;

    
    // skip white spaces 
    while i < n && into_byte[i] == ' ' {
        i += 1;
    }

    //if string contain  only spaces 
    if i == n { return 0;}

    //check for sign character 
    let mut sign = 1;
    if into_byte[i] == '-' {
        sign = -1;
    }

    // if they equal to this then  increase index
    if into_byte[i] == '-' || into_byte[i] == '+' {
        i += 1;
    }

    let mut result = 0;
    let overflowLIMIT = std::i32::MAX / 10;

    while i < n {
        if !into_byte[i].is_digit(10) {
            break;
        }

        let digit = into_byte[i] as u8 - b'0';

        if (result > overflowLIMIT) || (result == overflowLIMIT && digit > 7) {
            if sign == 1 {return std::i32::MAX } else {return std::i32::MIN};
        }

        result = result * 10 + digit as i32;
        i += 1;
    }   

    result * sign

}

fn main() {
    let s = String::from("-234erg");
    let ans = my_atoi(s);
    println!("{}", ans);
}