use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let reader = handle.lines();

    let t: usize = reader.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let first_line = reader.next().unwrap().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        
        let sn = reader.next().unwrap().unwrap();
        let sm = reader.next().unwrap().unwrap();
        
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