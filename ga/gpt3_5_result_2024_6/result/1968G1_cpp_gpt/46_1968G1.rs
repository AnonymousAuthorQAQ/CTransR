use std::io::{self, BufRead};
use std::collections::VecDeque;

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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let s = lines.next().unwrap();
        
        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(&s);
        
        let mut f = |x: usize| -> bool {
            if x > n / l {
                return false;
            }
            
            let mut t = String::new();
            let mut cnt = 0;
            
            for i in (x..s.len()).step_by(x) {
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
            if f(mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        
        println!("{}", ls);
    }
}