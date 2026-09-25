


pub fn reorder_log_files(logs:Vec<String>) -> Vec<String> {

    if logs.is_empty() {
        return logs
    }

    let mut letter_logs:Vec<String> = Vec::new();
    let mut digit_logs:Vec<String> = Vec::new();
    for ch in logs {
       let (_, content) = ch.split_once(' ').unwrap();
        println!("{}", content);
        if content.chars().next().unwrap().is_ascii_digit() {
            digit_logs.push(ch);
        } else {
            letter_logs.push(ch);
        }
    }

    println!("{:?}", digit_logs);

    println!("before letter log: {:?}", letter_logs);

    letter_logs.sort_by(|a, b| {
        let (a_id, a_content) = a.split_once(' ').unwrap();
        let (b_id, b_content) = b.split_once(' ').unwrap();

        match a_content.cmp(b_content) {
            std::cmp::Ordering::Equal => a_id.cmp(b_id),
            ordering => ordering
        }
    });

    println!("after lettter logs: {:?}", letter_logs);

    
    letter_logs.extend(digit_logs);
    

    letter_logs
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