use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.char_indices().enumerate().skip(1) {
        let mut k = 0;
        while i + k < n && s.chars().nth(i + k) == s.chars().nth(c + k) {
            k += 1;
        }
        z[i] = k;
    }
    z
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let l: usize = line.next().unwrap().parse().unwrap();
        let r: usize = line.next().unwrap().parse().unwrap();
        let s = line.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;
        while left + 1 < right {
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

fn can_repeat(s: &str, len: usize, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i..i + len] == s[i..i + len].repeat(l - 1) {
            count += 1;
            i += len * (l - 1);
        } else {
            i += 1;
        }
    }
    count >= l - 1
}