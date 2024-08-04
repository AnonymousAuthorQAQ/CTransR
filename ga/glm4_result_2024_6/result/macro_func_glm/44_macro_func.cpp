fn main() {
    let a: i32;
    let b: i32;

    println!("Enter two integers separated by a space:");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split(' ').collect();
    a = parts[0].parse::<i32>().expect("Please enter a valid integer for a");
    b = parts[1].parse::<i32>().expect("Please enter a valid integer for b");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}

fn cppmax(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}