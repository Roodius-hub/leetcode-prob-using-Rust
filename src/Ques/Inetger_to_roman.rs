// basic rule 
// subtractive way 
//Reapeation Limit 

// num = 1075 => 1000 + 70 + 5
//               M  + LXX +  V => MLXXV
// I , X ,C , M => repeat 3 time , and V, L , D cant be repeat

pub fn into_to_roman(mut num: i32) -> String {

    // first create tuple or Array or vector of  
    //cs contains the symbols: ('M', 'CM', 'D', 'CD', 'C', 'XC', 'L', 'XL', 'X', 'IX', 'V', 'IV', 'I')
    // vs contains the values: (1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1)    

    let roman_symbols = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut result = vec![];
    for i in 0..roman_symbols.len() {
        while num >= values[i] {
            num -= values[i];
            result.push(roman_symbols[i]);
        }
    }

    result.iter().cloned().collect::<String>()
}


fn main(){
    let num:i32 = 2005;
    let ans:String = into_to_roman(num);
    println!("num: {} in Roman: {}", num, ans);
}