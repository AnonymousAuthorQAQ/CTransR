use std::io;
use std::io::prelude::*;

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

fn can_repeat(s: &str, x: usize, l: usize) -> bool {
    let mut cnt = 0;
    let mut i = x;
    while i < s.len() {
        if z_function(&s[i - x..i])[0] >= x {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    cnt >= l - 1
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace();
        let n: usize = nums.next().unwrap().parse().unwrap();
        let l: usize = nums.next().unwrap().parse().unwrap();
        let r: usize = nums.next().unwrap().parse().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let s = input.trim();

        let mut ls = 0;
        let mut rs = n + 1;
        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if can_repeat(s, mid, l) {
                ls = mid;
            } else {
                rs = mid;
            }
        }

        writeln!(handle, "{}", ls).unwrap();
    }
}