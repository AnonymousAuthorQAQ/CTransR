use std::io::{self, BufRead};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    
    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = usize::max(0, r - i + 1);
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
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s: String = lines.next().unwrap();
        
        let z = z_function(&s);
        let mut ls = 0;
        let mut rs = n + 1;
        
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
            
            cnt >= l - 1
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