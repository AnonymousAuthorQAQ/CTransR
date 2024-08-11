use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    let t: i32 = iterator.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line: String = iterator.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();
        let sn: String = parts.next().unwrap().parse().unwrap();
        let sm: String = parts.next().unwrap().parse().unwrap();
        
        let mut i: usize = 0;
        let mut j: usize = 0;
        
        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}