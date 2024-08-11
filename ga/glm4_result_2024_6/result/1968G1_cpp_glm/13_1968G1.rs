use std::io::{self, BufRead};
use std::str::FromStr;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for i in 1..n {
        if i <= z[i - 1] {
            z[i] = z[i - 1];
        } else {
            z[i] = std::cmp::max(0, z[i - 1] - i + 1);
            while i + z[i] < n && &s[z[i]..z[i] + i] == &s[i..i + z[i]] {
                z[i] += 1;
            }
        }
    }
    z[0] = n;
    z
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap();
        let n: usize = line.parse().unwrap();
        let l: usize = line.parse().unwrap();
        let r: usize = line.parse().unwrap();
        let s = lines.next().unwrap();

        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(s.as_str());

        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if (0..n / mid).any(|i| z[i] >= mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }

        println!("{}", ls);
    }
}