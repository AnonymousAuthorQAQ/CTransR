use std::io;

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter two numbers: ");
    io::stdin().read_line(&mut a).expect("Failed to read input.");
    io::stdin().read_line(&mut b).expect("Failed to read input.");

    let a: i32 = a.trim().parse().expect("Please enter a number.");
    let b: i32 = b.trim().parse().expect("Please enter a number.");

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}