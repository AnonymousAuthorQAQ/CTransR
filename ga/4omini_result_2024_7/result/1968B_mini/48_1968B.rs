use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = first_line[0];
        let m = first_line[1];
        
        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();
        
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