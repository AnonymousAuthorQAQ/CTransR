fn main() {
    let mut input = String::new();
    println!("Enter two numbers separated by space: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers = input.trim().split_whitespace();
    let a: i32 = numbers.next().unwrap().parse().expect("Invalid number");
    let b: i32 = numbers.next().unwrap().parse().expect("Invalid number");

    let result = if (a + b) < 5 { b } else { a };
    println!("Result: {}", result);
}