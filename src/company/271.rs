use std::vec;


#[derive(Debug, Clone)]
pub struct EncodeDecodeString {
    ans : String
}

impl EncodeDecodeString {
    pub fn  new() -> Self {
        EncodeDecodeString {ans:String::new()}
    }

    // encode 
    pub fn  encode(&mut self, s:Vec<String>) -> String {
        let n = s.len();
        // let mut result:String = String::new();
        let mut result:String = String::new();
        for i in 0..n {
            let len = s[i].len();
            println!("length: {}", len);

            result.push_str(&len.to_string());
            result.push('#');
            result.push_str(&s[i]);
        }
        result
    }

    pub fn decode(&self, s:String) -> Vec<String> { 
        let bytes = s.as_bytes(); 
        let n = bytes.len();
        let mut result:Vec<String> = Vec::new();
        let mut i  = 0;

        while i < n {
            let mut length = 0;
            while bytes[i] != b'#' {
                length = length * 10 + (bytes[i] - b'0') as usize;
                i += 1;    
            }

            i += 1;

            let word = String::from_utf8(bytes[i..i + length].to_vec()).unwrap();
            result.push(word);
            i += length
        }
        result
    }

}


fn main() {
    let mut encodedecodestring = EncodeDecodeString::new();

    let ans = encodedecodestring.decode(String::from("5#hello5#world"));
    println!("{:?}", ans);
}