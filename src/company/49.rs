use std::collections::HashMap;

pub fn  sort_s(s:&String) -> String {
    let mut chars:Vec<char> = s.chars().collect();

    chars.sort_unstable();

    let sorted_s:String = chars.into_iter().collect();

    sorted_s

}

pub fn group_anagrams(strs:Vec<String>) -> Vec<Vec<String>>
{   
    // key will be sorted string and value will be vector of string
    let mut map:HashMap<String, Vec<String>> = HashMap::new(); 
    for st in strs {
        let new_st = sort_s(&st);
        println!("sorted string: {}", new_st);
        map.entry(new_st).or_insert(Vec::new()).push(st);
    }
    let mut ans:Vec<Vec<String>>  = Vec::new();    


    for (key, value) in map {
        ans.push(value);
    }

    ans
}



fn main() {
    let  words = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
    let ans = group_anagrams(words);
    println!("{:?}", ans);

    // let anns = sort_s("ate".to_string());
    // println!("{}", anns);

}