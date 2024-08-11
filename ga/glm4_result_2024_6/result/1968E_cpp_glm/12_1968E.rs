use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    
    println!("1 1");
    println!("2 1");
    
    let mut t = n - 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    
    for i in 3..=n {
        println!("1 {}", i);
    }
}

fn main() {
    let mut t = 1;
    loop {
        solve();
        println!(""); // Print newline between test cases
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        t = match input.trim().parse::<i32>() {
            Ok(0) => break,
            Ok(val) => val,
            Err(_) => continue,
        };
    }
}