use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut nums = first_line.split_whitespace();
        let n: usize = nums.next().unwrap().parse().unwrap();
        let m: usize = nums.next().unwrap().parse().unwrap();
        
        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();
        
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