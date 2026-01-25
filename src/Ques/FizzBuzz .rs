// 3 - fizz 
// 5 -buzz
// both 3 and 5  - fizzBuzz
// n will  be count or limit 1 to n

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut answer_tuple:Vec<String> = Vec::<String>::new();

    if n < 0 {
        println!("Input Should be greater then 0 and integer")
    }    
        
    for i in 1..=n {
        
        if i % 3 == 0 && i % 5 == 0 {
            answer_tuple.push("FizzBuzz".to_string());
            // println!("FizzBuzz");
        } else if i % 5 == 0 {
            answer_tuple.push("Buzz".to_string());
            // println!("Buzz");
        } else if i % 3 == 0  {
            answer_tuple.push("Fizz".to_string());
            // println!("Fizz");
        } else {
            answer_tuple.push(i.to_string());
            // println!("{}",i);
        }
        
    }
    
    answer_tuple
}

fn  main(){
   let result =  fizz_buzz(10);
   
   for num in result {  
       println!("{}",num);
   }
}