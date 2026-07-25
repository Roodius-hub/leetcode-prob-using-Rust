

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = intervals.len();
    
    let mut ans:Vec<Vec<i32>> = Vec::new();

    intervals.sort();

    // if n <= 0   {
    //     return intervals;
    // }
        
    for i in 0..n {
        if let Some(prev) = ans.last_mut() {
            let prev_end = prev[1]; // end time of previous interval
            // comapare y with current interval
            let curr_st = intervals[i][0];
            if curr_st <= prev_end {
                // overlap -> merge  them 
                prev[1] = std::cmp::max(prev_end, intervals[i][1])
            } else {
                ans.push(intervals[i].clone());
            }
        } else {
            ans.push(intervals[i].clone());
        }
    }
    ans
}



fn main() {
    let intervals:Vec<Vec<i32>> = vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]];
    let ans = merge(intervals);

    print!("{:#?}", ans)
    
}