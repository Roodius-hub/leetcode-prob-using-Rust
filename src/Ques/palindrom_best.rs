
pub fn is_palindrom(x:i32) -> bool {
    let ls:Vec<char> = x.to_string().chars().collect();
    let size: usize = ls.len();
        
    for i in 0..(size/2) {
        if ls[i] != ls[size-1-i] {
            return false;
        }
    }
        
    true
    
}


fn main() {
   let ans = is_palindrom(121);
   println!("{}" ,ans);
}