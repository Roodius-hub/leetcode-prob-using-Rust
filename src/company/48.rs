// QUESTION 48 rotate image

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {

    if matrix.is_empty() {
        return;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut  transpose_matrix = vec![vec![0; rows]; cols];
    
    for i in 0..rows {
        for j in 0..cols {
            transpose_matrix[j][i] = matrix[i][j].clone();
        }
    }

    for i in 0..rows {
        transpose_matrix[i].reverse();
    }

    for i in 0..rows {
            matrix[i] = transpose_matrix[i].clone();
    }
        
    // println!("matrix func: {:?}", transpose_matrix);

}





fn main() {
    let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];

    rotate(&mut matrix);

    println!("Main function: {:?}", matrix);
        
    // print!("{:#?}", matrix);
    
}