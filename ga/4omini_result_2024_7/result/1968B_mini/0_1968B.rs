use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        
        input.clear();
        reader.read_line(&mut input).unwrap();
        let sn = input.trim();
        
        input.clear();
        reader.read_line(&mut input).unwrap();
        let sm = input.trim();
        
        let mut i = 0;
        let mut j = 0;
        
        while i < n && j < m {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}