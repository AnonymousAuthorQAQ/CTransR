use std::io::{self, Read};

fn solve(n: i32) {
    let t = n - 2;
    println!("1 1");
    println!("2 1");
    if t > 0 {
        println!("{} {}", n, n);
        println!("1 {}", n + 1);
        for i in 4..=n {
            println!("1 {}", i);
        }
    } else {
        for i in 3..=n {
            println!("1 {}", i);
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
    let lines: Vec<&str> = buffer.lines().collect();

    let t = lines[0].parse::<i32>().expect("Failed to parse t");

    for i in 1..=t {
        let n = lines[i].parse::<i32>().expect("Failed to parse n");
        solve(n);
    }
}