use std::io;
use std::io::prelude::*;

fn gcd(x: i64, y: i64) -> i64 {
    let mut a = x;
    let mut b = y;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn can(x: i64, mid: i64) -> bool {
    gcd(x, mid) + mid > mid
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");
    let t: i64 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let x: i64 = input.trim().parse().expect("Invalid input");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;

            if can(x, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        writeln!(stdout, "{}", ans).expect("Failed to write output");
    }
}