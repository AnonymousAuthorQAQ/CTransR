use std::io::{self, BufRead};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, &c) in s.char_indices().enumerate() {
        let mut l = i;
        let mut r = i;
        while r < n && s.chars().nth(r) == Some(c) {
            r += 1;
        }
        z[i] = r - i;
        if i + z[i] < n {
            let mut k = 0;
            while r + k < n && s.chars().nth(i + k) == s.chars().nth(r + k) {
                k += 1;
            }
            z[i] = k;
            r += k;
        }
        if r > i {
            l = i;
        }
    }
    z
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap();

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

fn can_repeat(s: &str, x: usize, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if x <= s.len() - i && s[i..i + x] == s[i + x..i + 2 * x] {
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