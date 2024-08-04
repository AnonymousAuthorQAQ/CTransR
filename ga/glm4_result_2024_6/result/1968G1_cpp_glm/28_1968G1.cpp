use std::io;
use std::io::prelude::*;
use std::str::FromStr;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.char_indices().skip(1) {
        if i <= z[i - 1] && z[i - (n - z[i - 1])] < z[i - 1] - i + 1 {
            z[i] = z[i - (n - z[i - 1])];
        } else {
            z[i] = std::cmp::max(0, z[i - 1] - i + 1);
            while i + z[i] < n && s.chars().nth(z[i]) == s.chars().nth(i + z[i]) {
                z[i] += 1;
            }
        }
    }
    z[0] = n;
    z
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let line1 = lines.next().unwrap();
        let mut parts = line1.split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let l = parts.next().unwrap().parse::<usize>().unwrap();
        let r = parts.next().unwrap().parse::<usize>().unwrap();

        let s = lines.next().unwrap();

        let mut ls = 0;
        let mut rs = n + 1;

        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if can_divide(s, mid, l) {
                rs = mid;
            } else {
                ls = mid;
            }
        }

        println!("{}", ls);
    }
}

fn can_divide(s: &str, x: usize, l: usize) -> bool {
    if x > s.len() / l {
        return false;
    }

    let mut cnt = 0;
    let mut i = 0;

    while i < s.len() {
        if z_function(s)[i] >= x {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
    }

    cnt >= l - 1
}