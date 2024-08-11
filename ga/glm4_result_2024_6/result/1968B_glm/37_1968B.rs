use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    
    // Read the number of test cases
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    // Loop through each test case
    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut tokens = line.trim().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let m: usize = tokens.next().unwrap().parse().unwrap();
        let sn = tokens.next().unwrap();
        let sm = tokens.next().unwrap();

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