use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = std::cmp::max(0, r - i + 1);
            while i + z[i] < n && s.chars().nth(z[i]) == s.chars().nth(i + z[i]) {
                z[i] += 1;
            }
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z[0] = n;
    z
}

fn can_repeat(s: &str, x: usize, l: usize) -> bool {
    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if z_function(&s[i..]).last().unwrap() >= x {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
        if cnt >= l - 1 {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut t = String::new();
    stdin.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap();

        let mut ls = 0;
        let mut rs = n + 1;
        while ls + 1 < rs {
            let mid = ls + rs >> 1;
            if can_repeat(s, mid, l) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        writeln!(stdout, "{}", ls).unwrap();
    }
}