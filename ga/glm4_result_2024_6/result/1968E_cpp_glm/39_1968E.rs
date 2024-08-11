use std::io;
use std::io::{stdout, Write};

fn solve() {
    let n: i32 = read_int();
    let mut t = n;
    print!("1 1\n");
    print!("2 1\n");
    t -= 2;
    if t > 0 {
        print!("{} {}\n", n, n);
        t -= 1;
    }
    for i in 3..3+t {
        print!("1 {}\n", i);
    }
}

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut t: i32 = read_int();
    while t > 0 {
        solve();
        t -= 1;
    }
}