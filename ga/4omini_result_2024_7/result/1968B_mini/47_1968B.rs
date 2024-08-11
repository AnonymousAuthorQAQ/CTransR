use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    
    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        // Read n and m, and the two strings sn and sm
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        
        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();
        
        let mut i = 0;
        let mut j = 0;

        // Compare characters from sn and sm
        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1;
            }
            j += 1;
        }
        
        // Print the result for this test case
        println!("{}", i);
    }
}