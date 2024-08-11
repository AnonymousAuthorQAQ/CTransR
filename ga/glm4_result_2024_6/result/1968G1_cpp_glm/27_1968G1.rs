use std::io::{self, BufRead};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = std::cmp::max(0, r - i + 1);
            while i + z[i] < n && s.chars().nth(z[i]) == s.chars().nth(i + z[i]) {
                z[i] += 1;
            }
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z[0] = n;
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
            let mid = (left + right) / 2;
            if can_divide(mid, l, r, s) {
                left = mid;
            } else {
                right = mid;
            }
        }

        println!("{}", left);
    }
}

fn can_divide(x: usize, l: usize, r: usize, s: &str) -> bool {
    if x > n / l {
        return false;
    }
    let mut count = 0;
    let mut i = x;
    while i < s.len() {
        if z[i] >= x {
            count += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    count >= l - 1
}