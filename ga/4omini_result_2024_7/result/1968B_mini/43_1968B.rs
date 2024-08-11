use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        
        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();
        
        let (mut i, mut j) = (0, 0);
        
        while i < n && j < m {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}