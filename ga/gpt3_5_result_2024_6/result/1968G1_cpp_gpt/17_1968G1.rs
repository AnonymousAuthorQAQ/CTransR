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
            while i + z[i] < n && s.chars().nth(z[i]).unwrap() == s.chars().nth(i + z[i]).unwrap() {
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
    
    let mut lines = input.trim().split('\n');
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let s = lines.next().unwrap().trim().to_string();
        
        let ls = 0;
        let rs = n + 1;
        let z = z_function(&s);
        
        let f = |x: usize| -> bool {
            if x > n / l {
                return false;
            }
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
            return cnt >= l - 1;
        };
        
        let mut ls = ls;
        let mut rs = rs;
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