use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut t = String::new();
    input.read_line(&mut t).unwrap();
    let t: i32 = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();

        let mut a = String::new();
        input.read_line(&mut a).unwrap();
        let a = a.trim();

        let mut b = String::new();
        input.read_line(&mut b).unwrap();
        let b = b.trim();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}