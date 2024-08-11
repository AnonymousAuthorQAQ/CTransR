use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let mut parts = buffer.split_whitespace();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let a = buffer.trim().to_string();
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let b = buffer.trim().to_string();
        
        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}