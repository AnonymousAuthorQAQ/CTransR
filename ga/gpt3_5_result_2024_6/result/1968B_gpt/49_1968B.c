use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let mut input = String::new();
    lines.next().unwrap().unwrap(); // Skip the first line with the number of test cases
    
    while let Some(Ok(line)) = lines.next() {
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();
        let sn: String = parts.next().unwrap().parse().unwrap();
        let sm: String = parts.next().unwrap().parse().unwrap();
        
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}