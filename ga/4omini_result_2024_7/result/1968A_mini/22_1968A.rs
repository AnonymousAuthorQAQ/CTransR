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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        input.clear(); // Clear input buffer for next read
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: i32 = input.trim().parse().expect("Please type a number!");
        let y = find_y(x);
        println!("{}", y);
    }
}