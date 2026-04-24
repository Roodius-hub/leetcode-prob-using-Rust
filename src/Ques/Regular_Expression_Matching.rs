

pub fn is_match(s: String, p: String) -> bool {
    if p.len() == 0 {
        return s.len() == 0 
    } 

    let new_s:Vec<char> = s.chars().collect();
    let new_p:Vec<char> = p.chars().collect();

    let mut first_char_matched:bool = false;

    if new_s.len() > 0 && (new_s[0] == new_p[0] || new_p[0] == '.') {
        first_char_matched = true;
    }

    if new_p.len() >= 2 && new_p[1] == '*' {
        let  not_take:bool = is_match(new_s.iter().collect(), new_p[2..].iter().collect());
        let  take:bool = first_char_matched && is_match(new_s[1..].iter().collect(), new_p.iter().collect());

        return not_take || take;  
    }

    return first_char_matched && is_match(new_s[1..].iter().collect(), new_p[1..].iter().collect())
}

fn main(){
    let s:String = String::from("aa");
    let p:String = String::from(".a");

    let ans = is_match(s, p);

    println!("{}", ans);

}