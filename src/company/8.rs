

pub fn my_atoi(s: String) -> i32 {

    // skip - and " " places from string convert it intot  number
    // and rounding ineteger

    let into_byte:Vec<char> = s.chars().collect();
    let n = into_byte.len();
    let mut i = 0;
    let mut result = 0;
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
        if into_byte[i].is_digit(10) {
            
        }
    }

    1

}

fn main() {

}