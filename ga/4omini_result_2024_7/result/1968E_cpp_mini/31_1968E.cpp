use std::io::{self, Write};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut output = vec![];
    output.push("1 1".to_string());
    output.push("2 1".to_string());
    
    let mut t = n - 2;
    if t > 0 {
        output.push(format!("{} {}", n, n));
        t -= 1;
    }
    
    for i in 3..(3 + t) {
        output.push(format!("1 {}", i));
    }

    for line in output {
        println!("{}", line);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}