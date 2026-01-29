//An integer is a palindrome when it reads the same forward and backward
pub fn is_palindrom(x:i32) -> bool {
    let  input_string = x.to_string();
    let  copied_string = input_string.clone();
        
    let mut reverse_string = String::new();
    
    // reversed string
    for ch in copied_string.chars().rev() {
        // println!("{}", ch);
        reverse_string.push(ch);
    }
                    
    // convert to vector 
    let v1:Vec<char> = input_string.chars().collect(); // now vector
    let v2: Vec<char> = reverse_string.chars().collect(); // vector
            
    if v1.len() != v2.len() { 
        println!("Vector length not same it cant be palindrom");
        return false
    } 
                
    for  i in 0..v1.len() {
        if v1[i] != v2[i] {
            println!("Mismatch at index{}: '{}' != '{}'",i, v1[i],v2[i]);
            return false;
        } 
    }
    
    true
}

fn main(){
    let ans = is_palindrom(121);
    println!("{}" ,ans);
}