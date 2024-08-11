use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let x: i32 = input.trim().parse().unwrap();
        let y = find_y(x);
        println!("{}", y);
    }
}