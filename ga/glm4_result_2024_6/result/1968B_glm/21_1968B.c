use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut split = line.split_whitespace();
        
        let n: i32 = split.next().unwrap().parse().unwrap();
        let m: i32 = split.next().unwrap().parse().unwrap();
        
        let sn = split.next().unwrap();
        let sm = split.next().unwrap();
        
        let mut i = 0;
        let mut j = 0;
        
        while i < n && j < m {
            if sn.chars().nth(i as usize) == sm.chars().nth(j as usize) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}