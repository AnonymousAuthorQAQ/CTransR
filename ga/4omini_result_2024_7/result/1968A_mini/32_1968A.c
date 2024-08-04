use std::io;

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut input = String::new();
    
    // Read the number of test cases
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");
    
    for _ in 0..t {
        input.clear(); // Clear the buffer for the next input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: i32 = input.trim().parse().expect("Please enter a number");

        let y = find_y(x);
        println!("{}", y);
    }
}