use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input = lines.next().unwrap();
        let mut parts = input.split_whitespace();
        
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();
        
        let sn = lines.next().unwrap();
        let sm = lines.next().unwrap();
        
        let (mut i, mut j) = (0, 0);
        
        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}