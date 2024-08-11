use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    println!("1 1");
    println!("2 1");

    let mut t = n - 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..=n {
        if t > 0 {
            println!("1 {}", i);
            t -= 1;
        }
    }
}

fn main() {
    let mut t = 1;
    loop {
        solve();
        println!(""); // Print newline between test cases

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        t = match input.trim().parse::<usize>() {
            Ok(0) => break,
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}