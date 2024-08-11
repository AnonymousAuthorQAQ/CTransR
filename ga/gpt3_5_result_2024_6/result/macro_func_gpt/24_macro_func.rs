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
    
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    
    let mut nums = input.trim().split_whitespace();
    let a: i32 = nums.next().unwrap().parse().unwrap();
    let b: i32 = nums.next().unwrap().parse().unwrap();

    let result = cppmax(a, b);
    println!("Result: {}", result);
}