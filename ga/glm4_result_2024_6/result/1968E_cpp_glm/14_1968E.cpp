use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please type a number!");

    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;

    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..=3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut t = 1;
    println!("Enter the number of test cases:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    t = input.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        solve();
    }
}