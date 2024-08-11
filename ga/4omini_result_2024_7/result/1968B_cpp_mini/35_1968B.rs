use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut split = first_line.split_whitespace();
        let x: i32 = split.next().unwrap().parse().unwrap();
        let k: i32 = split.next().unwrap().parse().unwrap();
        
        let a: String = lines.next().unwrap().unwrap();
        let b: String = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] as char == i {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}