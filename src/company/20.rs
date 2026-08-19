pub fn is_matching(last_ch:Option<&char>,curr_ch:char) -> bool {
    let last_char = last_ch.unwrap();

    return (*last_char == '(' && curr_ch == ')') || (*last_char == '{' && curr_ch == '}') || (*last_char == '[' && curr_ch == ']') 

}

pub fn is_valid(s: String) -> bool {
    let mut stack:Vec<char>  = Vec::new();

    for ch in s.chars() {
        if ch == '(' || ch == '{' || ch == '[' {
            stack.push(ch)
        } 
        else if stack.is_empty() || !is_matching(stack.last(), ch) {
            return false
        } else {
            stack.pop();
        }
    }
    stack.len() == 0
}



fn main() {
    let s = String::from("[()]");
    let ans = is_valid(s);
    println!("Ans: {}", ans);
}