
pub fn int_to_roman(mut nums:i32) -> String {
    
    let roman_symbols = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

    let mut result:String = String::new();
    for i in 0..roman_symbols.len() {
        while nums >= values[i] {
            nums -= values[i];
            result.push_str(roman_symbols[i]);
        }
    }
    result
}



fn main() {
    let nums = 58;
    let ans = int_to_roman(nums);
    println!("{}", ans);
}