use std::io;

fn solve(n: usize) {
    let mut t = n; // n choices
    println!("1 1");
    println!("2 1");
    t -= 2; // For two choices printed
    
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    
    for i in 3..(3 + t) { // Print the remaining choices
        println!("1 {}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut n_input = String::new();
        io::stdin().read_line(&mut n_input).expect("Failed to read line");
        let n: usize = n_input.trim().parse().expect("Please enter a number");
        solve(n);
    }
}