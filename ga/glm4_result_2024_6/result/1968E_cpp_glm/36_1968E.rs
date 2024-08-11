use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Invalid input");

    println!("1 1");
    println!("2 1");

    if n > 2 {
        println!("{} {}", n, n);
    }

    for i in 3..=n {
        println!("1 {}", i);
    }
}

fn main() {
    println!("Enter the number of test cases:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Invalid number of test cases");

    for _ in 0..t {
        solve();
    }
}