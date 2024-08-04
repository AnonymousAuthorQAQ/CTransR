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
    t = t.parse().expect("Please enter a valid number");

    while t > 0 {
        println!("Enter a number: ");
        let mut x = String::new();
        std::io::stdin().read_line(&mut x).expect("Failed to read line");
        let x: i32 = x.trim().parse().expect("Please enter a valid number");

        let y = find_y(x);
        println!("The result is: {}", y);

        t -= 1;
    }
}