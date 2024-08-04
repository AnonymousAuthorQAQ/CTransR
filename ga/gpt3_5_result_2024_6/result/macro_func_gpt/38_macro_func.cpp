use std::io;

fn cppmax(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        return y;
    } else {
        return x;
    }
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut a).expect("Failed to read input.");

    println!("Enter value for b: ");
    io::stdin().read_line(&mut b).expect("Failed to read input.");

    let a: i32 = a.trim().parse().expect("Please enter a number.");
    let b: i32 = b.trim().parse().expect("Please enter a number.");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}