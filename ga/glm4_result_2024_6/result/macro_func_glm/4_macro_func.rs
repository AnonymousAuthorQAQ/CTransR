fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter two integers: ");
    std::io::stdin().read_line(&mut a).expect("Failed to read line");
    std::io::stdin().read_line(&mut b).expect("Failed to read line");

    let a: i32 = a.trim().parse().expect("Please enter a valid integer");
    let b: i32 = b.trim().parse().expect("Please enter a valid integer");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}

fn cppmax(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}