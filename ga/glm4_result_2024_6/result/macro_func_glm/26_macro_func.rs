fn main() {
    let mut input = String::new();
    println!("Enter two numbers separated by a space: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please type a number!")).collect();

    let a = numbers[0];
    let b = numbers[1];

    let result = if (a + b) < 5 { b } else { a };
    println!("Result: {}", result);
}