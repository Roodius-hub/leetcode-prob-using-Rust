//You are given a 2D matrix with m rows and n columns.
//  Your task is to traverse the matrix in a spiral pattern and return all elements in the order they are visited.

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
    }
    
    let (mut top, mut bottom) = (0, matrix.len() - 1);
    let (mut left, mut right) = (0, matrix[0].len() - 1);
    let mut ans:Vec<i32> = Vec::new();

    while top <= bottom && left <= right {
        // left to right
        for i in left..=right {
            ans.push(matrix[top][i])
        }
        top +=1;
    
        // top to bottom 
        for i in top..=bottom {
            ans.push(matrix[i][right])
        }
        right -=1;
    
        // right to left
        if top <= bottom {
            for i in (left..=right).rev() {
                ans.push(matrix[bottom][i])
            }
            bottom -= 1;
        }

        // bottom to top
        if left <= right {
            for i in (top..=bottom).rev() {
            ans.push(matrix[i][left]);
            }
            left +=1;
        }
        
    }
    ans
}


fn main() {
    let matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
   let ans = spiral_order(matrix);

   println!("{:?}", ans);
   
}