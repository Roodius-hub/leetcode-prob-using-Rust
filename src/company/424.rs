
pub fn character_replacement(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let mut answer = 0;
    let mut freq = [0; 26];
    let mut left = 0;
    let mut max_freq = 0;

    for right in 0..bytes.len() {
        let index = (bytes[right] - b'A') as usize;
        freq[index] += 1;

        max_freq = max_freq.max(freq[index]);

        while (right - left + 1) as i32 - max_freq > k  {
              let left_index = (bytes[left] - b'A') as usize;
              freq[left_index] -= 1;
              left += 1;
        }
        answer = answer.max(right - left + 1)
    }

    answer as i32
}




fn main() {
    let s = String::from("ABAB");
    let ans = character_replacement(s, 2);
    println!("{}",ans);
}