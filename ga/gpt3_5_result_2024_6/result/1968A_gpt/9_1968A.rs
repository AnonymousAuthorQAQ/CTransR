use std::io;

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let x: i32 = input.trim().parse().expect("Invalid input");

        let y = find_y(x);
        println!("{}", y);
    }
}