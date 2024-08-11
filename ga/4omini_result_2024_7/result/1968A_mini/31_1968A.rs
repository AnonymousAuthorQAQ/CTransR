use std::io;

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Input should be an integer");

    for _ in 0..t {
        input.clear(); // Clear the input buffer
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: i32 = input.trim().parse().expect("Input should be an integer");
        let y = find_y(x);
        println!("{}", y);
    }
}