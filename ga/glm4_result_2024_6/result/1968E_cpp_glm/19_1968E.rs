use std::io::{self, Read};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Failed to parse input");

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
    loop {
        solve();
        println!("Enter the number of test cases or -1 to exit: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        t = input.trim().parse().expect("Failed to parse input");
        if t == -1 {
            break;
        }
    }
}