use std::io::{self, BufRead};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let stdin = io::stdin();
    
    let t: i32 = stdin.lock().lines()
        .next().unwrap().unwrap()
        .trim().parse().unwrap();
    
    for _ in 0..t {
        let x: i32 = stdin.lock().lines()
            .next().unwrap().unwrap()
            .trim().parse().unwrap();
        
        let y = find_y(x);
        println!("{}", y);
    }
}