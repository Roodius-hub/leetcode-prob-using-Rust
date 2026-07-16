pub struct Stack<T>{
    elements:Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements:Vec::new()
        }
    }
    
    pub fn push(&mut self, val:T) {
        self.elements.push(val);
    }

    pub  fn pop(&mut self) ->Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}

pub fn longest_valid_parentheses(s:String) -> i32 {
    let mut stack = Stack::<usize>::new();
    let mut n = s.len();

    let chars:Vec<char> = s.chars().collect();
        
    let mut paint = vec![false; n];

    for i in 0..chars.len() {
        if chars[i] == '(' {
            stack.push(i);
        } else {
            if let Some(j) = stack.pop() {
                paint[j] = true;
                paint[i] = true;
            }
        }
    }

    let mut max_len = 0;
    let mut current_len = 0;

    for &flag in &paint {
        if flag {
            current_len += 1;
            if current_len > max_len {
                max_len = current_len;
            }
        } else {
            current_len = 0;
        }
    }
    println!("{}", max_len);
    max_len
}


fn main() {
    let s = String::from(")()())");
    longest_valid_parentheses(s);
}