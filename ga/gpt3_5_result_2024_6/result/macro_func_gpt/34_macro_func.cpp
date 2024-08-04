use std::io::{self, Read};

fn rust_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        return y;
    }
    return x;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let nums: Vec<i32> = input.trim().split_whitespace()
                       .map(|x| x.parse().unwrap())
                       .collect();
    
    let a = nums[0];
    let b = nums[1];

    let result = rust_max(a, b);
    println!("Result: {}", result);
}