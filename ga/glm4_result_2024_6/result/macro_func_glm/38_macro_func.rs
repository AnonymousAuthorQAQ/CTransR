fn main() {
    let mut input = String::new();
    println!("Enter two integers separated by a space:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter a valid integer"))
        .collect();

    let (a, b) = (numbers[0], numbers[1]);

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