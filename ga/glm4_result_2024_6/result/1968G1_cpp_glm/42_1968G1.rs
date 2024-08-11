use std::io::{self, BufRead};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            z[i] = n - i;
        } else if i < z[i - 1] {
            z[i] = z[i - 1];
        } else {
            let mut j = i;
            while j + z[i] < n && &s[j..j + z[i]] == &s[i..i + z[i]] {
                z[i] += 1;
                j += 1;
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
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if can_divide(s, mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }

        println!("{}", left);
    }
}

fn can_divide(s: &str, x: usize, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if x + i <= s.len() && &s[i..i + x] == &s[i + x..i + 2 * x] {
            count += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    count >= l - 1
}