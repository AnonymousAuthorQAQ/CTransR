use std::io;
use std::io::prelude::*;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for i in 1..n {
        if i <= z[i - 1] {
            z[i] = z[i - 1];
        }
        while i + z[i] < n && &s[z[i]..z[i] + 1] == &s[i..i + 1] {
            z[i] += 1;
        }
    }
    z
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let mut parts = buffer.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        parts.next();
        let s = parts.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if can_repeat(s, mid, l, r) {
                left = mid;
            } else {
                right = mid;
            }
        }

        writeln!(handle, "{}", left).unwrap();
    }
}

fn can_repeat(s: &str, len: usize, l: usize, r: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if z_function(&s[i..i + len]) >= len {
            count += 1;
            i += len;
        } else {
            i += 1;
        }
        if count >= l - 1 {
            return true;
        }
    }
    false
}