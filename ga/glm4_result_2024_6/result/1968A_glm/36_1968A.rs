fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut t = 0;
    println!("Enter the number of test cases: ");
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    let t: i32 = t.parse().expect("Please enter a number");

    for _ in 0..t {
        println!("Enter an integer: ");
        let mut x = String::new();
        std::io::stdin().read_line(&mut x).expect("Failed to read line");
        let x: i32 = x.trim().parse().expect("Please enter a valid integer");

        let y = find_y(x);
        println!("The result is: {}", y);
    }
}