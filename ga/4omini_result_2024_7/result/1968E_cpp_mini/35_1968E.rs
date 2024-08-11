use std::io::{self, Write};

fn solve() {
    let mut input = String::new();
    let n: usize = {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please type a number")
    };

    let mut output = Vec::new();

    output.push("1 1".to_string());
    output.push("2 1".to_string());

    let mut t = n - 2;

    if t > 0 {
        output.push(format!("{} {}", n, n));
        t -= 1;
    }

    for i in 3..(3 + t) {
        output.push(format!("1 {}", i));
    }

    for line in output {
        println!("{}", line);
    }
}

fn main() {
    let mut input_string = String::new();

    io::stdin().read_to_string(&mut input_string).expect("Failed to read input");

    let inputs: Vec<&str> = input_string.lines().collect();
    let test_cases: usize = inputs[0].trim().parse().expect("Please type a number");

    for case in 0..test_cases {
        let n: usize = inputs[case + 1].trim().parse().expect("Please type a number");
        solve(n);
    }
}