use std::io::{self, Read};
use std::str::FromStr;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut iter = lines.next().unwrap().split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s = lines.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if can_find_substring(s, mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left);
    }
}

fn can_find_substring(s: &str, x: usize, l: usize) -> bool {
    if x > s.len() / l {
        return false;
    }
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if z_function(&s[i..]).last().unwrap() >= x {
            count += 1;
            i += x;
        } else {
            i += 1;
        }
        if count >= l - 1 {
            return true;
        }
    }
    false
}