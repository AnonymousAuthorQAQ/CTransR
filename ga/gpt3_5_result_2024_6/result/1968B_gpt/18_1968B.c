use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();
    let t: i32 = input_line.trim().parse().unwrap();

    for _ in 0..t {
        input_line.clear();
        input.read_line(&mut input_line).unwrap();
        let mut parts = input_line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        input_line.clear();
        input.read_line(&mut input_line).unwrap();
        let sn = input_line.trim();

        input_line.clear();
        input.read_line(&mut input_line).unwrap();
        let sm = input_line.trim();
        
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