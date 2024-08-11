use std::io::{self, BufRead};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
}

    return gcd(b, a % b);
}

fn find_y(x: i32) -> i32 {
    return x - 1;
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    let t: i32 = iterator.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let x: i32 = iterator.next().unwrap().unwrap().trim().parse().unwrap();
        let y = find_y(x);
        println!("{}", y);
    }
}