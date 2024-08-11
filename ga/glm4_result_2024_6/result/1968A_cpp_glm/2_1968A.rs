use std::io;
use std::io::prelude::*;

fn can(x: i64, mid: i64) -> bool {
    let gcd = gcd::gcd(x, mid);
    gcd >= 1 + mid
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: i64 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let x: i64 = line.trim().parse().unwrap();

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

        writeln!(stdout, "{}", ans).unwrap();
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}