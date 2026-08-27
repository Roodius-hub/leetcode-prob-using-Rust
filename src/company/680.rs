
pub fn valid(s:&[char], l:usize, r:usize, deleted:bool) -> bool {

    if l >= r {
        return true;
    }

    if s[l] == s[r] {
        return valid(s, l + 1, r - 1, deleted);
    }

    if deleted {
        return false;
    }

    valid(s, l + 1, r, true) || valid(s, l, r - 1, true)

}

pub fn valid_palindrome(s:String) -> bool {
    let new_s:Vec<char> = s.to_lowercase().chars().filter(|c| !c.is_alphanumeric()).collect();

    if new_s.is_empty() {
        return true;
    }
    
    let l:usize = 0;
    let r = new_s.len() - 1;

    valid(&new_s, l, r, false)
}


fn main() {
    let s = String::from("abca");
    let ans = valid_palindrome(s);
    println!("{}", ans);
}