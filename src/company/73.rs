

// bturer force approach
// pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
//     if matrix.is_empty() {
//         return;
//     }

//     let (m, n) = (matrix.len(), matrix[0].len());
    
//     for i in 0..n {
//         for j in 0..m {
//                 if matrix[i][j] == 0 {
//                     markrow(i, m as i32,matrix);
//                     markcol(j, n as i32, matrix);
//                 }
//         }
//     }


//     fn markrow(i:usize, m:i32, matrix: &mut Vec<Vec<i32>>) {
//         for j in 0..m {
//             if matrix[i][j as usize] != 0 {
//                 matrix[i][j as usize] = -1; 
//             }
//         }
//     }
    
//     fn  markcol(j:usize, n:i32,  matrix: &mut Vec<Vec<i32>>) {
//         for i in 0..n {
//             if matrix[i as usize][j] != 0 {
//                 matrix[i as usize][j] = -1;
//             }
//         }
//     }

//     for i in 0..n {
//         for j in 0..m {
//             if matrix[i][j] == -1 {
//                 matrix[i][j] = 0;
//             }
//         }
//     }
// }

// better approach
// pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
//     if matrix.is_empty() {
//         return;
//     }
    
//     let (m ,n) = (matrix.len(), matrix[0].len());
//     let (mut row, mut col) = (vec![false; m], vec![false; n]); // its a row size array and col size array 

//     // marking in row and col  vector true false
//     for i in 0..n {
//         for j in 0..m {
//             if matrix[i][j] == 0 {
//                 row[i] = true;
//                 col[j] = true;
//             }
//         }
//     }
//     // converting into 0
//     for i in 0..n {
//         for j in 0..m {
//             if row[i] == true || col[j] == true {
//                 matrix[i][j] = 0;
//             }
//         }
//     }
// }



pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    if matrix.is_empty() {
        return;
    }
    
    let (m ,n) = (matrix.len(), matrix[0].len());
    // let (mut row, mut col) = (vec![false; m], vec![false; n]); // its a row size array and col size array 

    // marking in row and col  vector true false
    let mut col0 = 1;
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                // we mark the i-th row
                matrix[i][0] = 0;
                // we mark the j-th col
                if j != 0 {
                    matrix[0][j] = 0;
                } else {
                    col0 = 0;
                }
            }
        }
    }
    // converting into 0
    for i in 1..m {
        for j in 1..n {
                if matrix[i][j] != 0 {
                    if matrix[i][0] == 0 || matrix[0][j] == 0 {
                        matrix[i][j] = 0;
                    }
                }
        }
    }

    if matrix[0][0] == 0 {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }

    if col0 == 0 {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }
    
}

fn main(){ 
    let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    set_zeroes(&mut matrix);

    println!("{:?}",matrix);
}