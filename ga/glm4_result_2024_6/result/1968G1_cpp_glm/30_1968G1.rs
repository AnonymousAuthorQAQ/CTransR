use std::io::{self, BufRead};
use std::cmp::min;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            z[i] = n - i;
        } else {
            let mut k = min(z[i - 1], n - i);
            while i + k < n && &s[i..i + k] == &s[k..k + k] {
                k += 1;
            }
            z[i] = k;
        }
    }
    z
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    
    for _ in 0..t {
        let line1 = lines.next().unwrap().unwrap();
        let mut parts = line1.split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let l = parts.next().unwrap().parse::<usize>().unwrap();
        let r = parts.next().unwrap().parse::<usize>().unwrap();
        let s = lines.next().unwrap().unwrap();

        let mut left = 0;
        let mut right = n + 1;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if can_divide(s.as_str(), mid, l, r) {
                left = mid;
            } else {
                right = mid;
            }
        }

        println!("{}", left);
    }
}

fn can_divide(s: &str, len: usize, l: usize, r: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    let mut z = z_function(s);
    while i < s.len() {
        if z[i] >= len {
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