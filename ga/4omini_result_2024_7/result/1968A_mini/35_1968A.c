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
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().expect("Failed to read t");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let x: i32 = line.trim().parse().expect("Failed to read x");

        let y = find_y(x);
        println!("{}", y);
    }
}