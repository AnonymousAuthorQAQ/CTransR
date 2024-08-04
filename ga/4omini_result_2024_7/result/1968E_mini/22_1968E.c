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
    
    for i in 3..(3 + t) {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read the number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        // Read value of n for each test case
        stdin.read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        solve(n);
    }
}