use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    println!("1 1");
    println!("2 1");

    let mut t = n - 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..=3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut t = 1;
    loop {
        solve();
        println!("Enter the number of test cases: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        t = input.trim().parse().unwrap();
        if t == 0 {
            break;
        }
    }
}