
pub fn convert(s: String, num_rows: i32) -> String {
    // in a two rows zigzag not possible
    if num_rows == 1 { 
        return s;
    }   

    let num_rows = num_rows as usize;

    let mut my_array = vec![vec![]; num_rows];
    let mut i = 0;
    let mut k = false;

    for c in s.chars() {
        // println!("{}", c);
        my_array[i].push(c);
        if i == 0 || i == num_rows - 1 {
            k =  !k
        } 
        if k {
            i += 1;
        } else {
            i -= 1;
        }
        
    }
    // concate all chars
    let mut result = String::new(); 
    for row in my_array {
        for ch in row {
            result.push(ch);
        }
    }

    result
}


fn main() {
    let my_str:String = String::from("PAYPALISHIRING");
    let num_rows = 3;
    let  ans = convert(my_str, num_rows);

    print!("{:?}", ans);


}