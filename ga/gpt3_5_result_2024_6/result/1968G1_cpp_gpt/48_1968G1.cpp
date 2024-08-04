use std::io::{self, BufRead};
use std::cmp::{max, min};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    
    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = max(0, r - i + 1);
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

    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input_line: Vec<usize> = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = input_line[0];
        let l = input_line[1];
        let r = input_line[2];

        let s = lines.next().unwrap().trim().to_string();

        let mut ls = 0;
        let mut rs = n + 1;

        let z = z_function(&s);

        let f = |x: usize| -> bool {
            if x > n / l {
                false
            } else {
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
            }
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