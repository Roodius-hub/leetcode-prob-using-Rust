use std::collections::HashMap;


pub fn is_valid(x:String) -> bool {
    println!("Enter Parentheses String only:");
    let x_vec:Vec<char> = x.chars().collect(); // into vec
    let mut stack:Vec<char> = Vec::new(); // stack
    // stored in unordered_map
    let mut open_close:HashMap<char, char> = HashMap::new();
    open_close.insert('(',')');
    open_close.insert('[',']');
    open_close.insert('{','}');    
                        
    
    for ch in x_vec {
        if open_close.contains_key(&ch) {
            stack.push(ch);  
        } else {
            if let Some(top) = stack.pop() {
                if open_close.get(&top) != Some(&ch) {
                   return false;
                }
            } else {
                  return  false; 
            }
        }
    }
    
    stack.is_empty()
}

fn main() {
    let s:String = String::from("([}])");
    let ans = is_valid(s);

    println!("{}", ans);
}