

pub fn valid(s:&[char], l:usize, r:usize) -> bool {
    if l >= r {
        return true;
    }

    if s[l] != s[r] {
        return false;
    }

    valid(s, l+1, r-1)
}

pub fn is_palindrome(s:String) -> bool {
    let new_s:Vec<char> = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    
    if new_s.is_empty() {
        return true
    }

    let l:usize = 0;
    let r:usize = new_s.len() - 1;
    valid(&new_s, l, r)
}



fn main() {
    let s = String::from("A man, a plan, a canal: Panama");
    let ans = is_palindrome(s);
    println!("{}", ans);
}