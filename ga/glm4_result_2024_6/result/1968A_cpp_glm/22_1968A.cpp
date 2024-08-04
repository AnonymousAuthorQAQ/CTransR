use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut x_input = String::new();
        io::stdin().read_line(&mut x_input).expect("Failed to read line");
        let x: u64 = x_input.trim().parse().expect("Invalid input");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = l + r >> 1; // equivalent to (l + r) / 2

            if can(x, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        println!("{}", ans);
    }
}

fn can(x: u64, mid: u64) -> bool {
    let gcd = gcd(x, mid);
    gcd + mid <= x
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}