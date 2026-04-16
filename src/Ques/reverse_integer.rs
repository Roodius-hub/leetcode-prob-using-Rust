

pub fn reverse(x: i32) -> i32 {
    // covert into string
    let x_string = x.to_string();
    let into_chars:Vec<char> = x_string.chars().collect();  
    // for c in into_chars {
    //     println!("{}", c);
    // }      

    if into_chars[0 as usize] == '-' {
        println!("Number are Negative");
        return -1
    } else {
        let num = (into_chars.reverse().join()).parse::<i32>().expect("Not a Number");
        
    }

    3
}

fn main() {
    println!("Hi there from reverse integer");
    let x = 123;
    let ans = reverse(x);
}