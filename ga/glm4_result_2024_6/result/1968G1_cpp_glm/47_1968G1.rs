use std::io;
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
        let z = z_function(s);

        while ls + 1 < rs {
            let mid = ls + rs / 2;
            if can_repeat(s, mid, l, &z) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}

fn can_repeat(s: &str, x: usize, l: usize, z: &[usize]) -> bool {
    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if z[i] >= x {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    cnt >= l - 1
}