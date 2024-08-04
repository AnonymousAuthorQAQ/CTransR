use std::io::{self, Read};

fn solve(n: usize) {
    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    for i in 3..=t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let t: usize = input.trim().parse().expect("Invalid input for t");

    for _ in 0..t {
        let mut n_input = String::new();
        io::stdin().read_line(&mut n_input).expect("Failed to read from stdin");
        let n: usize = n_input.trim().parse().expect("Invalid input for n");
        solve(n);
    }
}