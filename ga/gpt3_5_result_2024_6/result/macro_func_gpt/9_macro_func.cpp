use std::io;

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        return y;
    } else {
        return x;
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter two integers:");
    
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let nums: Vec<i32> = input.split_whitespace()
                               .map(|num| num.parse().expect("Invalid input"))
                               .collect();

    let a = nums[0];
    let b = nums[1];

    let result = cpp_max(a, b);
    
    println!("Result: {}", result);
}