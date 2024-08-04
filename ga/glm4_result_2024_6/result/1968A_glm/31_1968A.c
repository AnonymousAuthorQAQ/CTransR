use std::io;

fn decrement(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut x_str = String::new();
        io::stdin().read_line(&mut x_str).expect("Failed to read line");
        let x: i32 = x_str.trim().parse().expect("Please enter a number");

        let y = decrement(x);
        println!("{}", y);
    }
}