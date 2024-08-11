use std::io::{self, Write};
use num::integer::gcd;

fn can(mid: i64, x: i64) -> bool {
    gcd(x, mid) + mid > 0 // Check if the combination is positive
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse");

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let x: i64 = input.trim().parse().expect("Failed to parse");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;
            if can(mid, x) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        writeln!(handle, "{}", ans).expect("Failed to write");
    }
}