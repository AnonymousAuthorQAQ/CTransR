use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a number");

    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;

    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..=n as usize {
        if t > 0 {
            println!("1 {}", i);
            t -= 1;
        } else {
            break;
        }
    }
}

fn main() {
    println!("Enter the number of test cases: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        solve();
    }
}