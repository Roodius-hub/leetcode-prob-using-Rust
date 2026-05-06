

fn main() {
    let mut  ans:i64 = 0;
    let num:i64 = 699999;
    for i in 0..=num  {
        println!("{}", i);
        ans += i;
    }   

    println!("{}", ans);

}