use std::io;
use std::io::{BufRead, Write};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, _) in s.chars().enumerate().skip(1) {
        let mut l = i;
        let mut r = i;
        while r < n && s.chars().nth(r) == s.chars().nth(r - l) {
            r += 1;
        }
        z[i] = r - i;
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z[0] = n;
    z
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let s = line.trim();

        let mut left = 0;
        let mut right = n + 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if can_find(s.as_str(), mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }
        writeln!(stdout, "{}", left).unwrap();
    }
}

fn can_find(s: &str, x: usize, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if x <= s.len() - i && z_function(&s[i..]).first() >= &x {
            count += 1;
            i += x;
        } else {
            i += 1;
        }
        if count >= l - 1 {
            return true;
        }
    }
    false
}