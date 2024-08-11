use std::io;

fn cppmax(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        return y;
    } else {
        return x;
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter two numbers:");
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let a = numbers[0];
    let b = numbers[1];

    let result = cppmax(a, b);
    println!("Result: {}", result);
}