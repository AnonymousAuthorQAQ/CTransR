use std::io;

fn solve(n: usize) {
    let mut t = n;

    println!("1 1");
    println!("2 1");
    t -= 2;

    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..3+t {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input for t");

    for _ in 0..t {
        input.clear();  // Clear buffer for new input
        stdin.read_line(&mut input).expect("Failed to read line");
        let n: usize = input.trim().parse().expect("Invalid input for n");
        solve(n);
    }
}