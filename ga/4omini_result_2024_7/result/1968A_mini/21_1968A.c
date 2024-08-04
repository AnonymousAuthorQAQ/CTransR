use std::io::{self, BufRead};

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let t: i32 = reader.lines().next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let x: i32 = reader.lines().next().unwrap().unwrap().parse().unwrap();
        let y = find_y(x);
        println!("{}", y);
    }
}