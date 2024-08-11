use std::io;

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
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read input");
    let t: i32 = t.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read input");
        let x: i32 = x.trim().parse().expect("Invalid input");

        let y = find_y(x);
        println!("{}", y);
    }
}