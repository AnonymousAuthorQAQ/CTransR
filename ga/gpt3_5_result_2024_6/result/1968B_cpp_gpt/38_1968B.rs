use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = lines.next().unwrap().trim().split_whitespace();
        let x: i32 = input.next().unwrap().parse().unwrap();
        let k: i32 = input.next().unwrap().parse().unwrap();
        let a: String = lines.next().unwrap().trim().to_string();
        let b: String = lines.next().unwrap().trim().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if let Some(ch) = a.chars().nth(ans as usize) {
                if ch == i {
                    ans += 1;
                }
            }
        }

        println!("{}", ans);
    }
}