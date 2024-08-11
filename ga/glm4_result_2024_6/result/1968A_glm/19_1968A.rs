fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut t = 0;
    println!("Enter the number of test cases: ");
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    println!("Enter the test cases: ");
    for _ in 0..t {
        let mut x = 0;
        std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
        let y = find_y(x);
        println!("Output: {}", y);
    }
}