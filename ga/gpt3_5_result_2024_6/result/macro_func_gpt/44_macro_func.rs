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
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();
    
    let a = nums[0];
    let b = nums[1];
    
    let result = cppmax(a, b);
    println!("Result: {}", result);
}