use std::io;
use std::io::prelude::*;
use std::str;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for i in 1..n {
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
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line1 = lines.next().unwrap();
        let mut parts = line1.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = lines.next().unwrap();

        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(s);

        while ls + 1 < rs {
            let mid = ls + rs >> 1;
            if can_repeat(mid, &z, n, l) {
                ls = mid;
            } else {
                rs = mid;
            }
        }

        writer.write_all(format!("{}\n", ls).as_bytes()).unwrap();
    }
}

fn can_repeat(x: usize, z: &Vec<usize>, n: usize, l: usize) -> bool {
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