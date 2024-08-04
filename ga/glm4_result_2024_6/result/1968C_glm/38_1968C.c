use std::io;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut x = Vec::with_capacity(n);
    for _ in 1..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let xi: usize = input.trim().parse().expect("Invalid input");
        x.push(xi);
    }

    let mut a = Vec::with_capacity(n);
    a.push(x[0] + 1);

    for i in 1..n {
        let mut j = 0;
        loop {
            a.push(x[i] + j * a[i - 1]);
            if i + 1 < n && x[i + 1] < a[i] || a[i] != a[i - 1] {
                break;
            }
            j += 1;
        }
    }

    for ai in a {
        println!("{}", ai);
    }
}

fn main() {
    println!("Enter the number of test cases:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        proc();
    }
}