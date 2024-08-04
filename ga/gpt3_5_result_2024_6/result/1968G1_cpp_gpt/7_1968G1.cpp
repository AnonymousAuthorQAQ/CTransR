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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = input[0];
        let l = input[1];
        let r = input[2];

        let s = lines.next().unwrap();

        let mut ls = 0;
        let mut rs = n + 1;

        let z = z_function(&s);

        let find = |x: usize| -> bool {
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
            if find(mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}