use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();
    let t: usize = input_line.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input_line = String::new();
        input.read_line(&mut input_line).unwrap();
        let mut iter = input_line.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();
        
        let mut sn = String::new();
        input.read_line(&mut sn).unwrap();
        let sn = sn.trim();
        
        let mut sm = String::new();
        input.read_line(&mut sm).unwrap();
        let sm = sm.trim();
        
        let (mut i, mut j) = (0, 0);
        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}