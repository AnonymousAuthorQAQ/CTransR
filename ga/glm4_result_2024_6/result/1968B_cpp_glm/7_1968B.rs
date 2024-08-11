use std::io;
use std::io::{Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<usize>().expect("Invalid test case count");

    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();

        let x: usize = parts.next().unwrap().parse().expect("Invalid x value");
        let k: usize = parts.next().unwrap().parse().expect("Invalid k value");

        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        let mut ans = 0;

        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}