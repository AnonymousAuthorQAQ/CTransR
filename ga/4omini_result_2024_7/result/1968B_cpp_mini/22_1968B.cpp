use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let k: i32 = parts.next().unwrap().parse().unwrap();
        
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();

        let mut ans = 0;
        for char_b in b.chars() {
            if ans < a.chars().count() && a.chars().nth(ans).unwrap() == char_b {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}