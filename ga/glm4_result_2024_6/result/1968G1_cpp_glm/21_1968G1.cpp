use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    while t > 0 {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = lines.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;
        let mut z = z_function(s);

        while left + 1 < right {
            let mid = (left + right) / 2;
            if is_valid(mid, n, l, r, &z, s) {
                left = mid;
            } else {
                right = mid;
            }
        }

        println!("{}", left);

        t -= 1;
    }
}

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    z[0] = n;

    for i in 1..n {
        if i <= z[i - 1] {
            z[i] = z[i - 1];
        } else {
            z[i] = max(0, z[i - 1] - i + 1);
            while i + z[i] < n && s.chars().nth(z[i]) == s.chars().nth(i + z[i]) {
                z[i] += 1;
            }
        }
    }

    z
}

fn is_valid(mid: usize, n: usize, l: usize, r: usize, z: &Vec<usize>, s: &str) -> bool {
    if mid > n / l {
        return false;
    }

    let mut count = 0;
    let mut i = mid;
    while i < s.len() {
        if z[i - mid] >= mid {
            count += 1;
            i += mid;
        } else {
            i += 1;
        }
    }

    count >= l - 1
}

fn max<T: Ord>(a: T, b: T) -> T {
    std::cmp::max(a, b)
}