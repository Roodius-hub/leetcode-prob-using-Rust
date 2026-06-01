pub fn generate_parentheses(n:i32) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();

    generate(n, String::from(""), 0, 0, &mut result);
    result
}



fn generate(n:i32, current:String, open:i32, close:i32, ans: &mut Vec<String>) {
    if current.len() == (2 * n) as usize {
        ans.push(current);
        return;
    }

    if open < n {
        let mut next = current.clone();
        next.push('(');
        generate(n, next, open + 1, close, ans);
    }

    if close < open {
        let mut next = current.clone();
        next.push(')');
        generate(n, next, open, close +  1, ans);
    }

}


fn main() {
    let  ans = generate_parentheses(3);
    for v in ans {
        println!("combination: {}", v);
    }
}