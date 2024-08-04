use std::io::{self, Write};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

macro_rules! read {
    ($t:ty) => {
        input.trim().parse::<$t>().unwrap()
    };
    ([$t:ty]) => {
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    };
}

fn z_function(s: &str) -> Vec<i64> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = 0.max(r - i + 1);
            while i + z[i] < n && s.as_bytes()[z[i]] == s.as_bytes()[i + z[i]] {
                z[i] += 1;
            }
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z[0] = n as i64;
    z
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: i64 = read!(i64);
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let parts: Vec<i64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let l = parts[1];
        let r = parts[2];
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let s = input.trim().to_string();
        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(&s);
        let mut f = |x: i64| -> bool {
            if x > n / l {
                return false;
            }
            let mut t = String::new();
            let mut cnt = 0;
            let mut i = x;
            while i < s.len() as i64 {
                if z[i as usize] >= x {
                    cnt += 1;
                    i += x;
                } else {
                    i += 1;
                }
            }
            return cnt >= l - 1;
        };
        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if f(mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}