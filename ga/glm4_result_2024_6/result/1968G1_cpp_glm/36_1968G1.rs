use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();

    while t > 0 {
        let (n, l, r): (usize, usize, usize) = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_tuple().unwrap();
        let s = lines.next().unwrap();
        
        let mut z = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i <= r && z[i - l] < r - i + 1 {
                z[i] = z[i - l];
            } else {
                z[i] = std::cmp::max(0, r - i + 1);
                while i + z[i] < n && s.chars().nth(i + z[i]) == s.chars().nth(i) {
                    z[i] += 1;
                }
            }
            if i + z[i] - 1 > r {
                l = i;
                r = i + z[i] - 1;
            }
        }
        z[0] = n;

        let mut ls = 0;
        let mut rs = n + 1;

        while ls + 1 < rs {
            let mid = ls + (rs - ls) / 2;
            if is_divisible(mid, &s, l, &z) {
                ls = mid;
            } else {
                rs = mid;
            }
        }

        println!("{}", ls);

        t -= 1;
    }
}

fn is_divisible(mid: usize, s: &str, l: usize, z: &[usize]) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if z[i] >= mid {
            count += 1;
            i += mid;
        } else {
            i += 1;
        }
        if count >= l - 1 {
            return true;
        }
    }
    false
}