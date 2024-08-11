use std::io;
use std::io::prelude::*;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for i in 1..n {
        if i <= z[i - 1] {
            z[i] = z[i - 1];
        } else {
            z[i] = 0;
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
    let mut stdout = io::stdout();

    let mut t = String::new();
    stdin.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap();

        let mut ls = 0;
        let mut rs = n + 1;

        let z = z_function(s);

        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if can_form(&z, mid, l, n) {
                ls = mid;
            } else {
                rs = mid;
            }
        }

        write!(stdout, "{}\n", ls).unwrap();
    }
}

fn can_form(z: &[usize], x: usize, l: usize, n: usize) -> bool {
    let mut i = 0;
    let mut count = 0;
    while i < n {
        if z[i] >= x {
            count += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    count >= l - 1
}