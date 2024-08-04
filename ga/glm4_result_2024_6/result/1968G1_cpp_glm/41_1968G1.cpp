use std::io::{self, BufRead};

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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let n: usize = line.split_whitespace().next().unwrap().parse().unwrap();
        let l: usize = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        let r: usize = line.split_whitespace().nth(2).unwrap().parse().unwrap();
        let s: String = line.split_whitespace().nth(3).unwrap().to_string();

        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(&s);

        while ls + 1 < rs {
            let mid = ls + rs >> 1;
            if (0..n/mid).any(|i| z[i*mid] >= mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }

        println!("{}", ls);
    }
}