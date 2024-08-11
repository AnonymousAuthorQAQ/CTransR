fn solve(n: i32) {
    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    for i in 3..=n {
        println!("1 {}", i);
    }
}

fn main() {
    let mut t = 1;
    println!("Enter the number of test cases:");
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    println!("Enter the values:");
    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: i32 = input.trim().parse().expect("Please enter a number");
        solve(n);
    }
}