use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let k: i32 = parts.next().unwrap().parse().unwrap();
        let a: String = parts.next().unwrap().to_string();
        let b: String = parts.next().unwrap().to_string();
        
        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans as usize) == Some(i) {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}