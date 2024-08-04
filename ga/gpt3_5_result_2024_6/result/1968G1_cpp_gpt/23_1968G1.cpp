use std::io::{self, BufRead};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    
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
    let mut input = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = input.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line: Vec<usize> = input.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = line[0];
        let l = line[1];
        let r = line[2];
        let s = input.next().unwrap().trim().to_string();
        
        let z = z_function(&s);
        
        let mut ls = 0;
        let mut rs = n + 1;
        
        let f = |x: usize| -> bool {
            if x > n / l { return false; }
            let mut cnt = 0;
            for i in (x..s.len()).step_by(x) {
                if z[i] >= x {
                    cnt += 1;
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