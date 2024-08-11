use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a number");
    
    print!("1 1\n");
    print!("2 1\n");
    
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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    t = input.trim().parse().expect("Please enter a number");
    
    while t > 0 {
        solve();
        t -= 1;
    }
}