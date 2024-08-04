use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    // Read number of test cases
    stdin.lock().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    input.clear();

    for _ in 0..t {
        input.clear();

        // Read input values for each test case
        stdin.lock().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace();
        let n: usize = values.next().unwrap().parse().unwrap();
        let m: usize = values.next().unwrap().parse().unwrap();
        
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let sn = input.trim();

        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let sm = input.trim();

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