use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let t: i32 = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut input = lines.next().unwrap().split_whitespace();
        let x: i32 = input.next().unwrap().parse().unwrap();
        let k: i32 = input.next().unwrap().parse().unwrap();

        let a: String = lines.next().unwrap();
        let b: String = lines.next().unwrap();

        let mut ans = 0;
        for i in b.chars() {
            if let Some(c) = a.chars().nth(ans as usize) {
                if c == i {
                    ans += 1;
                }
            }
        }
        
        println!("{}", ans);
    }
}