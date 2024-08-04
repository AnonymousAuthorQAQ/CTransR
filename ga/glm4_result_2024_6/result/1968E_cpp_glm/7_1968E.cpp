use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse input");

    println!("1 1");
    println!("2 1");
    if n > 2 {
        println!("{} {}", n, n);
        for i in 3..=n {
            println!("1 {}", i);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse input");

    for _ in 0..t {
        solve();
    }
}