use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    let t: i32 = buffer.trim().parse().expect("Invalid input");

    for _ in 0..t {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line");
        let mut input = buffer.trim().split_whitespace();
        let x: i32 = input.next().unwrap().parse().expect("Invalid input");
        let k: i32 = input.next().unwrap().parse().expect("Invalid input");

        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line");
        let mut input = buffer.trim().split_whitespace();
        let a: String = input.next().unwrap().to_string();
        let b: String = input.next().unwrap().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }
        writeln!(stdout, "{}", ans).expect("Failed to write");
    }
}