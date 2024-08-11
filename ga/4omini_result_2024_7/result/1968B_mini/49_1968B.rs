use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let n = first_line[0];
        let m = first_line[1];
        
        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();

        let mut i = 0; // index for sn
        let mut j = 0; // index for sm
        
        while i < n && j < m {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}