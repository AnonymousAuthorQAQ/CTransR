use std::io::{self, BufRead};
use std::str;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, &c) in s.char_indices().enumerate() {
        if i == 0 {
            z[i] = n - i;
        } else {
            let mut j = i;
            while j < n && s.chars().nth(j) == Some(c) {
                j += 1;
            }
            z[i] = j - i;
            if i + z[i] < n {
                let mut k = 1;
                while i + k < n && k + z[i + k] < j {
                    z[i + k] = z[i + k - j + i];
                    k += 1;
                }
                j = i + k;
                while i + k < n && s.chars().nth(i + k) == s.chars().nth(j) {
                    z[i + k] += 1;
                    k += 1;
                    j += 1;
                }
            }
        }
    }
    z
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let l = parts.next().unwrap().parse::<usize>().unwrap();
        let r = parts.next().unwrap().parse::<usize>().unwrap();
        let s = parts.next().unwrap();
        let mut left = 0;
        let mut right = n + 1;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if can_repeat(mid, s, l) {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left);
    }
}

fn can_repeat(length: usize, s: &str, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i + length <= s.len() {
        count += 1;
        i += length;
    }
    count >= l - 1
}