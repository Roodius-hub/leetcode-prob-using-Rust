
pub fn is_match(s: String, p: String) -> bool {
    let s:Vec<char> = s.chars().collect();
    let p:Vec<char> = p.chars().collect();

    let mut dp = vec![vec![None; p.len() + 1]; s.len() + 1];

    fn dfs(i:usize, j:usize, s:&Vec<char>, p:&Vec<char>, dp:&mut Vec<Vec<Option<bool>>>) -> bool {
        if let Some(val) = dp[i][j] {
            return val;
        }

        // length  checking
        let ans = if j == p.len() { 
            i == s.len()
        } else {
            let first_match = i < s.len() && (s[i] == p[j] || p[j] == '.');
            //skip char* or not
            if j + 1 < p.len() && p[j+1] == '*' {
                dfs(i, j+2, s, p ,dp) || dfs(i+1, j, s, p, dp)
            } else {
                first_match && dfs(i + 1, j + 1, s, p , dp)
            }
 
        };
        dp[i][j] = Some(ans);
        ans
    }

    dfs(0, 0, &s, &p, &mut dp)
}



fn main() {
    let s = "aab".to_string();
    let p = "c*a*b".to_string();

    println!("{}", is_match(s, p));
}