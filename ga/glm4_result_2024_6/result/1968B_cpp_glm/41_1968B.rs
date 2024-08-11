use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut line = lines.next().unwrap().unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line = lines.next().unwrap().unwrap();
        let mut parts = line.trim().split_whitespace();
        let x_str = parts.next().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        let mut ans = 0;
        for (i, &c) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(i).unwrap() == c {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}