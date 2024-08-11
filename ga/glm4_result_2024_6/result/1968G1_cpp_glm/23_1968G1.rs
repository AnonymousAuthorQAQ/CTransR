use std::io;
use std::io::prelude::*;
use std::str;

fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for i in 1..n {
        if i <= z[i - 1] {
            z[i] = z[i - 1];
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
    }
    z
}

fn f(s: &[u8], x: usize) -> bool {
    if x > s.len() / x {
        return false;
    }
    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if z[i] >= x {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    cnt >= x - 1
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = str::parse(&line).unwrap();

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut nums = line.split_whitespace();
        let n: usize = str::parse(nums.next().unwrap()).unwrap();
        let l: usize = str::parse(nums.next().unwrap()).unwrap();
        let r: usize = str::parse(nums.next().unwrap()).unwrap();
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let s = line.as_bytes();

        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(s);
        while ls + 1 < rs {
            let mid = ls + rs >> 1;
            if f(s, mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        write!(stdout, "{}\n", ls).unwrap();
    }
}