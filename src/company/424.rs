
pub fn character_replacement(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();



    let mut answer = 0;


    for l in 0..bytes.len() {
        // let index = (bytes[l] - b'A') as usize;
        // freq[index] += 1;
        for r in l+1..bytes.len() {
            let mut freq = [0; 26];

            for i in l..=r {

                let index = (bytes[i] - b'A') as usize;
                freq[index] += 1;
            }

            let max_freq = *freq.iter().max().unwrap();
            let window_size = r - l + 1;
            let replacement_needed = window_size - max_freq;
 
            if replacement_needed as i32 <= k {
                answer = answer.max(window_size);
            }
        }
    }

    answer as i32
}




fn main() {
    let s = String::from("ABAB");
    let ans = character_replacement(s, 2);
    println!("{}",ans);
}