

pub fn reverse(x: i32) -> i32 {
    // covert into string
    let x_string = x.to_string();
    let into_chars:Vec<char> = x_string.chars().collect();  
    // for c in into_chars {
    //     println!("{}", c);
    // }      
    let mut is_negative = into_chars.first() == Some(&'-');
    let mut num = match into_chars.into_iter().filter(|&c| c != '-').rev().collect::<String>().parse::<i64>()
    {
        Ok(n) => n,
        Err(_) =>  0
    };


    if is_negative {
        num = -num;
    } 

    let result:i32 = if num > i32::MAX as i64 || num < i32::MIN as i64 {
        0 as i32
    } else {
        num as i32
    };
    result
}

fn main() {
    println!("Hi there from reverse integer");
    let x = 123;
    let ans = reverse(x);
    println!("{}", ans);
}