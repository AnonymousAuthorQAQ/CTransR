use std::io::{self, Write};
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
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inputs: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let n = inputs[0];
        let l = inputs[1];
        let r = inputs[2];

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        let (mut ls, mut rs) = (0, n + 1);
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