use std::io::{self, Write};

fn solve(n: i32) {
    let mut output = vec![];

    output.push(format!("1 1"));
    output.push(format!("2 1"));
    let mut t = n - 2;

    if t > 0 {
        output.push(format!("{} {}", n, n));
        t -= 1;
    }

    for i in 3..3 + t {
        output.push(format!("1 {}", i));
    }

    // Print all collected output at once
    println!("{}", output.join("\n"));
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    // Read the number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        solve(n);
    }
}