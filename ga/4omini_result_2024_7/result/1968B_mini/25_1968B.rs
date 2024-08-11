use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        let sn: &str = &lines.next().unwrap().unwrap();
        let sm: &str = &lines.next().unwrap().unwrap();
        
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