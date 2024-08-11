use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    let t = n;
    println!("1 1");
    println!("2 1");
    let mut t = t - 2;

    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..=t + 3 {
        println!("1 {}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        solve();
    }
}