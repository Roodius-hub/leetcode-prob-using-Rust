use std::vec;


pub fn plus_one(digits: Vec<i32>) -> Vec<i32>{
    let  digitAsString = digits.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    let  mut digitsAsNum: i32 = digitAsString.parse().unwrap();
        
    digitsAsNum += 1;
    
    // println!("{}" ,digitsAsNum);
        
   let spliteddigits:Vec<i32> =  digitsAsNum.to_string().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
   
   // print!("{:?}",spliteddigits );
   
   spliteddigits
   
    // println!("{}", stringAsNum);
    // println!("{}", digitAsString)
    
    // for num in digits {
    //     println!("{}", num);
    // } 
}

fn main(){
    let digits = vec![9];
    let ans:Vec<i32>  = plus_one(digits);
    
    println!("{:?}", ans);
}