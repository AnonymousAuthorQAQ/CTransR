macro_rules! cppmax {
    ($x:expr, $y:expr) => {
        if $x + $y < 5 {
            $y
        } else {
            $x
        }
    };
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    // Read input values from stdin
    std::io::stdin().read_line(&mut a).unwrap();
    std::io::stdin().read_line(&mut b).unwrap();

    // Parse the input strings to integers
    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();

    // Compute the result using the macro
    let result = cppmax!(a, b);
    
    // Print the result
    println!("Result: {}", result);
}