


pub fn reorder_log_files(logs:Vec<String>) -> Vec<String> {
    
}


fn main(){
    let logs: Vec<String> = vec![
    "dig1 8 1 5 1".to_string(),
    "let1 art can".to_string(),
    "dig2 3 6".to_string(),
    "let2 own kit dig".to_string(),
    "let3 art zero".to_string(),
];

    let ans = reorder_log_files(logs);
    println!("{:?}", ans);
}