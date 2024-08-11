use std::io::{self, Read};
use std::str::FromStr;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, &c) in s.char_indices().enumerate() {
        if i == 0 {
            z[i] = n;
        } else {
            let mut l = 0;
            let mut r = 0;
            while r < n - i && s.chars().nth(r) == Some(c) {
                r += 1;
            }
            z[i] = r;
            r -= 1;
            if i + z[i] > r {
                l = i;
                r = i + z[i] - 1;
            }
            while i + z[i] + l <= n && s.chars().nth(i + z[i] + l) == Some(c) {
                z[i] += 1;
            }
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
        let s = parts.next().unwrap();
        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(s);
        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if can_repeat(s, &z, mid, l, r) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}

fn can_repeat(s: &str, z: &[usize], x: usize, l: usize, r: usize) -> bool {
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
    count >= l - 1 && (n / x) <= r
}