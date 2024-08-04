use std::io::{self, Read};

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

fn f(s: &str, x: usize) -> bool {
    if x > s.len() / x {
        return false;
    }
    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if z_function(s[i..]).first() >= &x {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    cnt >= x - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap();
        let n: usize = line.split_whitespace().next().unwrap().parse().unwrap();
        let l: usize = line.split_whitespace().next().unwrap().parse().unwrap();
        let r: usize = line.split_whitespace().next().unwrap().parse().unwrap();
        let s = lines.next().unwrap();
        let mut ls = 0;
        let mut rs = n + 1;
        while ls + 1 < rs {
            let mid = ls + rs / 2;
            if f(s, mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}