use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    
    // Parse the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        
        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();
        
        let mut i = 0;
        let mut j = 0;
        
        // Count characters in sn that can be found in sm in order
        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1;
            }
            j += 1;
        }
        
        // Output the result for the current test case
        println!("{}", i);
    }
}