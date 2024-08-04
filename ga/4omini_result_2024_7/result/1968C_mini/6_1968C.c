use std::io;

fn process() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut x = vec![0; n];
    let mut a = vec![0; n];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input.trim().split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();

    // Copy input into `x`, starting from index 1
    for i in 1..n {
        x[i] = numbers[i - 1]; // `x[1]` is numbers[0]
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=1_000_000_000 {
            a[i] = x[i] + j * a[i - 1];

            // Check conditions for breaking the inner loop
            let next_x_value = if i + 1 < n { x[i + 1] } else { 1 };
            if next_x_value < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Print the result
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        process();
    }
}