use std::io;
use std::cmp::Ordering;
use std::io::{BufRead, Write};

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

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) == 1
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: u64 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let x: u64 = line.trim().parse().unwrap();

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;

            match can(x, mid).cmp(&true) {
                Ordering::Equal => {
                    ans = mid;
                    l = mid + 1;
                }
                Ordering::Greater => {
                    r = mid - 1;
                }
                _ => {
                    l = mid + 1;
                }
            }
        }

        writeln!(handle, "{}", ans).unwrap();
    }
}