
// pub fn divide(mut dividend: i32, divisor: i32) -> i32 {
//     let mut count =0;

//     while divisor <= dividend {
//         dividend -= divisor;
//         count +=1;
//     }

//     count
// }


pub fn divide(mut dividend: i32, divisor: i32) -> i32 {
    if dividend == i32::MIN  && divisor == -1 {
        return i32::MAX;
    }

    // negative sign
    let negative = (dividend < 0) ^ (divisor < 0);

    let mut dividend = (dividend  as i64).abs();
    let divisor = (divisor  as i64).abs();

    let mut quotient  = 0i64;

    while dividend >= divisor {
        let mut shift = 0;

        while dividend >= (divisor << (shift + 1)) {
            shift += 1;
        }

        dividend -= divisor << shift;

        quotient += 1 << shift

    }

    if negative {
        -(quotient as i32)
    } else {
        quotient as i32
    }    
}


fn main() {
    let ans = divide(10, 3);
    println!("{}",ans);
}