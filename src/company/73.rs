

// bturer force approach
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    if matrix.is_empty() {
        return;
    }

    let (m, n) = (matrix.len(), matrix[0].len());

    // let (row, col) = (vec![false; m], vec![false; n]);
    
    for i in 0..n {
        for j in 0..m {
                if matrix[i][j] == 0 {
                    markrow(i, m as i32,matrix);
                    markcol(j, n as i32, matrix);
                }
        }
    }


    fn markrow(i:usize, m:i32, matrix: &mut Vec<Vec<i32>>) {
        for j in 0..m {
            matrix[i][j as usize] = -1;
        }
    }
    
    fn  markcol(j:usize, n:i32,  matrix: &mut Vec<Vec<i32>>) {
        for i in 0..n {
            matrix[i as usize][j] = -1;
        }
    }

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == -1 {
                matrix[i][j] = 0;
            }
        }
    }
}


fn main(){ 
    let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    set_zeroes(&mut matrix);

    println!("{:?}",matrix);
}