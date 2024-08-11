use std::io::{self, Read};
use std::str;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, _) in s.chars().enumerate().skip(1) {
        let mut l = 0;
        let mut r = 0;
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
    let t = lines.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let l = parts.next().unwrap().parse::<usize>().unwrap();
        let r = parts.next().unwrap().parse::<usize>().unwrap();
        let s = lines.next().unwrap();
        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(s);
        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if (0..=n/mid).any(|x| {
                let mut cnt = 0;
                let mut i = x;
                while i < s.len() {
                    if z[i] >= mid {
                        cnt += 1;
                        i += mid;
                    } else {
                        i += 1;
                    }
                }
                cnt >= l - 1
            }) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}