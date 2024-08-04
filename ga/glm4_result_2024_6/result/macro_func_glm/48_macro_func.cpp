fn main() {
    let mut input = String::new();
    println!("Enter two integers (a b):");

    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    if let [a, b] = numbers.as_slice() {
        let result = cppmax(*a, *b);
        println!("Result: {}", result);
    } else {
        println!("Please provide exactly two integers.");
    }
}

fn cppmax(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}