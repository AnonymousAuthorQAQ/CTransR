use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.chars().enumerate() {
        if i > 0 {
            let mut k = z[i - 1];
            while k > 0 && c != s.chars().nth(k) {
                k = z[k - 1];
            }
            if c == s.chars().nth(k) {
                k += 1;
            }
            z[i] = k;
        }
    }
    z
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = lines.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;

        while left + 1 < right {
            let mid = (left + right) / 2;
            if can_repeat(s, mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }

        println!("{}", left);
    }
}

fn can_repeat(s: &str, len: usize, min_repeats: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i <= s.len() - len {
        if s[i..i + len] == s[i + len..i + 2 * len] {
            count += 1;
            i += len;
        } else {
            i += 1;
        }
    }
    count >= min_repeats - 1
}