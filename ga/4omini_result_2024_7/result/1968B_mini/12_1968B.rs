use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader for stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of test cases
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        // Read the dimensions and the strings for each test case
        let dims: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
        let n = dims[0];
        let m = dims[1];
        
        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();
        
        let (mut i, mut j) = (0, 0);
        // Two-pointer approach to find matching characters
        while i < n && j < m {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1; // Move i if there's a match
            }
            j += 1; // Always move j
        }
        
        // Output the result for the current test case
        println!("{}", i);
    }
}