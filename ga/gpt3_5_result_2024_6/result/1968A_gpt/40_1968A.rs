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
    let mut buffer = String::new();

    stdin.lock().read_line(&mut buffer).unwrap();
    let t: i32 = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let x: i32 = buffer.trim().parse().unwrap();

        let y = find_y(x);
        println!("{}", y);
    }
}