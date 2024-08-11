use std::io::{self, Read};
use std::cmp::min;

fn z_function(s: &str) -> Vec<i64> {
    let n = s.len();
    let mut z = vec![0; n];
    let s = s.as_bytes();
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = 0.max(r - i + 1);
            while i + z[i] < n && s[z[i]] == s[i + z[i]] {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let t: i64 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().trim();
        let mut parts = line.split_whitespace();
        let n: i64 = parts.next().unwrap().parse().unwrap();
        let l: i64 = parts.next().unwrap().parse().unwrap();
        let r: i64 = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap();
        
        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(s);
        
        let mut f = |x: i64| -> bool {
            if x > n / l {
                return false;
            }
            let mut cnt = 0;
            let mut i = x as usize;
            while i < s.len() {
                if z[i] >= x {
                    cnt += 1;
                    i += x as usize;
                } else {
                    i += 1;
                }
            }
            return cnt >= l - 1;
        };
        
        while ls + 1 < rs {
            let mid = ls + rs >> 1;
            if f(mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}