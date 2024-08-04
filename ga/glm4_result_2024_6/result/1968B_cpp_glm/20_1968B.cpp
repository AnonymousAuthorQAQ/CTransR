use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut input = String::new();
    let mut writer = io::BufWriter::new(stdout.lock());

    stdin.read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();
        let a: String = iter.next().unwrap().to_string();
        let b: String = iter.next().unwrap().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }

        writeln!(writer, "{}", ans).unwrap();
    }
}