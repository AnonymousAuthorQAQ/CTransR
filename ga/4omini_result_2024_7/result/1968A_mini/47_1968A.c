use std::io;

fn find_y(x: i32) -> i32 {
    return x - 1;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let t: usize = input.trim().parse().expect("Invalid number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let x: i32 = input.trim().parse().expect("Invalid number");
        let y = find_y(x);
        println!("{}", y);
    }
}