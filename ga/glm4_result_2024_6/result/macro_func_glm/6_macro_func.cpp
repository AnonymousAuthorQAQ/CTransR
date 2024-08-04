fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    let a: i32 = parts.next().expect("No number found").parse().expect("Invalid number");
    let b: i32 = parts.next().expect("No number found").parse().expect("Invalid number");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}

fn cppmax(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}