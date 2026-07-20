

// Best Time to Buy and Sell Stock


pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_prof = 0;
    let mut min_price = std::i32::MAX;

    for i in 0..prices.len() {
        min_price = std::cmp::min(min_price, prices[i]);
        max_prof = std::cmp::max(max_prof, prices[i] - min_price);
    }
    max_prof   
}

fn  main() {
    let  prices = vec![7,1,5,3,6,4];

    let ans = max_profit(prices);
    println!("{}", ans);
}