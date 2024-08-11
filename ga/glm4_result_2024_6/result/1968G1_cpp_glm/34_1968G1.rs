use std::io::{self, Read};
use std::str::FromStr;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, &c) in s.chars().enumerate() {
        if i > 0 {
            z[i] = if i <= z[i - 1] {
                if s[z[i - 1]..].starts_with(s[i..]) {
                    z[i - 1] + z_function(s[z[i - 1]..]).first().unwrap()
                } else {
                    0
                }
            } else {
                if s[..n - i].starts_with(s[i..]) {
                    (n - i) + z_function(&s[i..n - i]).first().unwrap()
                } else {
                    0
                }
            };
        }
    }
    z
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        parts.next();
        let s = parts.next().unwrap();
        let mut left = 0;
        let mut right = n + 1;
        while left < right - 1 {
            let mid = left + (right - left) / 2;
            if can_repeat(s, mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left);
    }
}

fn can_repeat(s: &str, x: usize, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if x <= s.len() - i && &s[i..i + x] == &s[i + x..i + 2 * x] {
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