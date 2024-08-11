use std::io::{self, Read};
use std::str::FromStr;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.char_indices().skip(1) {
        if i <= z[i - 1] && &s[i..i + z[i - 1]] == &s[..z[i - 1]] {
            z[i] = z[i - 1];
        } else {
            z[i] = match s[i..].starts_with(&s[..n - i]) {
                true => n - i,
                false => 0,
            };
            while i + z[i] < n && &s[z[i]..z[i] + n - i] == &s[i..i + z[i]] {
                z[i] += 1;
            }
        }
    }
    z
}

fn f(s: &str, x: usize, l: usize) -> bool {
    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if x <= s.len() - i && &s[i..i + x] == &s[..x] {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    cnt >= l - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        let mut parts = input.lines().next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: String = parts.next().unwrap().to_string();
        let mut ls = 0;
        let mut rs = n + 1;
        while ls + 1 < rs {
            let mid = ls + rs / 2;
            if f(&s, mid, l) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}