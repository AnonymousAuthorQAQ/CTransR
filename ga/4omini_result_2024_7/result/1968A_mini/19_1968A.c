use std::io;

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut x_input = String::new();
        io::stdin().read_line(&mut x_input).unwrap();
        let x: i32 = x_input.trim().parse().unwrap();

        let y = find_y(x);
        println!("{}", y);
    }
}