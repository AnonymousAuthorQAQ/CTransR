use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: i32 = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line: String = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();
        let sn: String = parts.next().unwrap();
        let sm: String = parts.next().unwrap();
        
        let mut i = 0;
        let mut j = 0;
        
        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }
        
       println!("{}", i);
    }
}