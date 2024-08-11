use std::io::{self, Write};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let s: Vec<char> = s.chars().collect();
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = std::cmp::max(0, r - i + 1);
            while i + z[i] < n && s[z[i]] == s[i + z[i]] {
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
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s = input.trim();

        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(s);
        
        let f = |x: usize| -> bool {
            if x > n / l {
                return false;
            }
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