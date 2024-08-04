use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let l: usize = line.next().unwrap().parse().unwrap();
        let r: usize = line.next().unwrap().parse().unwrap();
        let s: String = line.next().unwrap().to_string();
        let mut z = z_function(&s);
        let mut ls = 0;
        let mut rs = n + 1;
        while ls + 1 < rs {
            let mid = ls + rs >> 1;
            if f(&z, mid, l, r) {
                ls = mid;
            } else {
                rs = mid;
            }
        }
        println!("{}", ls);
    }
}

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for i in 1..n {
        if i <= z[i - 1] {
            z[i] = z[i - 1];
        } else {
            z[i] = max(0, z[i - 1] + 1);
            while i + z[i] < n && s.chars().nth(z[i]) == s.chars().nth(i + z[i]) {
                z[i] += 1;
            }
        }
    }
    z[0] = n;
    z
}

fn f(z: &Vec<usize>, x: usize, l: usize, r: usize) -> bool {
    if x > n / l {
        return false;
    }
    let mut cnt = 0;
    let mut i = x;
    while i < n {
        if z[i] >= x {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    cnt >= l - 1
}

fn max<T: Ord>(a: T, b: T) -> T {
    std::cmp::max(a, b)
}