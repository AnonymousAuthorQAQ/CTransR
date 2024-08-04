use std::io::{self, Read};

fn solve() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let n: usize = buffer.trim().parse().expect("Please enter a number");
    
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
    let t: usize = loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        match buffer.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a number"),
        }
    };
    
    for _ in 0..t {
        solve();
    }
}