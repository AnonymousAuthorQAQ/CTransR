use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            z[i] = n;
        } else {
            let mut l = 0;
            let mut r = 0;
            while r < n - i && s.chars().nth(r) == s.chars().nth(r + i) {
                r += 1;
            }
            z[i] = r;
            if r > 0 {
                l = i;
                r *= -1;
            }
            while i + z[i - l] < n && s.chars().nth(i + z[i - l]) == s.chars().nth(l + z[i - l]) {
                z[i] += z[i - l];
            }
        }
    }
    z
}

fn f(s: &str, x: usize, l: usize) -> bool {
    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if x > s.len() / l {
            return false;
        }
        if z_function(s[i..]).first() >= &x {
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
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = lines.next().unwrap();
        let mut ls = 0;
        let mut rs = n + 1;
        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if f(s, mid, l) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}