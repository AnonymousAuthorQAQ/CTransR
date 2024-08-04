use std::io;

fn solve(n: i32) {
    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    for i in 3..3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut t: i32 = input.trim().parse().expect("Invalid input");

    while t > 0 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let n: i32 = input.trim().parse().expect("Invalid input");
        solve(n);
        t -= 1;
    }
}