use std::io::{self, Write};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

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
            while i + z[i] < n && s.as_bytes()[z[i]] == s.as_bytes()[i + z[i]] {
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
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        let mut ls = 0;
        let mut rs = n + 1;

        let z = z_function(s);
        let mut f = |x: usize| -> bool {
            if x > n / l { return false; }
            let mut cnt = 0;
            let mut i = x;
            while i < s.len() {
                if z[i] >= x {
                    cnt += 1;
                    i += x;
                } else {
                    i += 1;
                }
            }
            cnt >= l - 1
        };

        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            match f(mid) {
                true => ls = mid,
                false => rs = mid,
            }
        }
        println!("{}", ls);
    }
}