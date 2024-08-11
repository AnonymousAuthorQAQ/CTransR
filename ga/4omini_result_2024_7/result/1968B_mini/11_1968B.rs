use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read number of test cases
    let mut input_lines = reader.lines();
    let t: usize = input_lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        // Read n and m as well as the two strings
        let first_line = input_lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();
        
        let sn: String = input_lines.next().unwrap().unwrap();
        let sm: String = input_lines.next().unwrap().unwrap();

        // Initialize indices for sn and sm
        let mut i = 0;
        let mut j = 0;

        // Count the number of characters from sn that can be found in sm
        while i < n && j < m {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1;
            }
            j += 1;
        }

        // Print the result for the current test case
        println!("{}", i);
    }
}