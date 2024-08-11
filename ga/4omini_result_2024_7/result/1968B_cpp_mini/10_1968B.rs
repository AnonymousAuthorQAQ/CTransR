use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();
        
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        for c in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] as char == c {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}