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
    let mut input_buffer = String::new();

    stdin.lock().read_line(&mut input_buffer).unwrap();
    let t: i32 = input_buffer.trim().parse().unwrap();

    for _ in 0..t {
        input_buffer.clear();
        stdin.lock().read_line(&mut input_buffer).unwrap();
        let x: i32 = input_buffer.trim().parse().unwrap();
        
        let y = find_y(x);
        println!("{}", y);
    }
}