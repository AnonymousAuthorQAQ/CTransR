use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a number");

    println!("1 1");
    println!("2 1");

    let mut t = n - 2;
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
    loop {
        solve();
        println!("Enter the number of test cases or type 'exit' to end: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }
        t = input.parse().expect("Please enter a number");
    }
}