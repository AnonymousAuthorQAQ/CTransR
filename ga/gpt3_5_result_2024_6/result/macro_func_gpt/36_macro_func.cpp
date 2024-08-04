use std::io;

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let mut values = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let a = values.next().unwrap_or(0);
    let b = values.next().unwrap_or(0);

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}