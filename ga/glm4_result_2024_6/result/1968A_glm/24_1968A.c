fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut t = 0;
    println!("Enter the number of test cases: ");
    std::io::stdin().read_line(&mut t).expect("Failed to read line");
    let t: i32 = t.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        println!("Enter a number: ");
        let mut x = String::new();
        std::io::stdin().read_line(&mut x).expect("Failed to read line");
        let x: i32 = x.trim().parse().expect("Please enter a number");

        let y = find_y(x);
        println!("Result: {}", y);
    }
}