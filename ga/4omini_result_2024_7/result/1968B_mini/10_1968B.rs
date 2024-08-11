use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line = lines.next().unwrap();
        let parts: Vec<usize> = first_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let n = parts[0]; // length of sn
        let m = parts[1]; // length of sm
        
        let sn = lines.next().unwrap();
        let sm = lines.next().unwrap();
        
        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1;
            }
            j += 1;              
        }
        
        println!("{}", i);
    }
}