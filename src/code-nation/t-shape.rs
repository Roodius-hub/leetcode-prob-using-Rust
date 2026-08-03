use std::collections::HashSet;

// | Orientation | Required points (relative to junction)         |
// | ----------- | ---------------------------------------------- |
// | Up          | (x-1, y), `(x+1, y)`, `(x, y+1)`, `(x, y+2)` |
// | Down        | (x-1, y)`, `(x+1, y)`, `(x, y-1)`, `(x, y-2)` |
// | Left        | (x, y-1)`, `(x, y+1)`, `(x-1, y)`, `(x-2, y)` |
// | Right       | (x, y-1)`, `(x, y+1)`, `(x+1, y)`, `(x+2, y)` |


/* For each test case:
    Read 5 points → Vec + HashSet
    found = false
    
    For each point (x, y) in Vec:
        If (Up pattern exists in HashSet)    → found = true, break
        If (Down pattern exists in HashSet)  → found = true, break
        If (Left pattern exists in HashSet)  → found = true, break
        If (Right pattern exists in HashSet) → found = true, break
    
    Print "Yes" if found, else "No" */

pub fn t_shape(points:Vec<(i32,i32)>) -> bool {
    let seen:HashSet<(i32,i32)> = points.iter().cloned().collect();
    
    for &(x,y) in &points {
        let up =  vec![(x-1, y), (x+1, y), (x, y+1), (x, y+2)];
        let down = vec![(x-1, y), (x+1, y), (x, y-1),(x, y-2)];
        let left = vec![(x, y-1), (x, y+1),(x-1, y),(x-2, y)];
        let right = vec![(x, y-1),(x, y+1),(x+1, y),(x+2, y)];
      
        if up.iter().all(|p| seen.contains(p)) {
            return true;
        }

        if down.iter().all(|p| seen.contains(p)) {
            return true;
        }

        if left.iter().all(|p| seen.contains(p)) {
            return true;
        }

        if right.iter().all(|p| seen.contains(p)) {
            return true;
        }        
    }
    false
}


fn main() {
   let matrix =  
        // Test 1: T pointing Up (Yes)
        vec![(5, 5), (4, 5), (6, 5), (5, 6), (5, 7)];
    
        // // Test 2: T pointing Down (Yes)
        // vec![(10, 10), (9, 10), (11, 10), (10, 9), (10, 8)],
    
        // // Test 3: T pointing Left (Yes)
        // vec![(2, 2), (2, 1), (2, 3), (1, 2), (0, 2)],
    
        // // Test 4: T pointing Right (Yes)
        // vec![(100, 100), (100, 99), (100, 101), (101, 100), (102, 100)],
    
        // // Test 5: Plus sign (No)
        // vec![(5, 5), (4, 5), (6, 5), (5, 4), (5, 6)],
    
        // // Test 6: Stem has a gap (No)
        // vec![(0, 0), (1, 0), (2, 0), (0, 1), (0, 3)],
    
        // // Test 7: Crossbar has a gap (No)
        // vec![(5, 5), (3, 5), (7, 5), (5, 6), (5, 7)],
    
        // // Test 8: Straight line (No)
        // vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
    
        // // Test 9: Random scatter (No)
        // vec![(1, 1), (5, 9), (3, 3), (7, 2), (0, 0)],
    
        // // Test 10: L-shape (No)
        // vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    

        if t_shape(matrix) {
            println!("YES")
        } else {
            println!("NO")
        }
   
}