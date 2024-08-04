use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, _) in s.chars().enumerate().skip(1) {
        let mut l = 0;
        let mut r = 0;
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = r - i + 1;
            while i + z[i] < n && s.chars().nth(i + z[i]) == s.chars().nth(z[i]) {
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
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let l: usize = line.next().unwrap().parse().unwrap();
        let r: usize = line.next().unwrap().parse().unwrap();
        let s = line.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if can_repeat(s, mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }

        println!("{}", left);
    }
}

fn can_repeat(s: &str, x: usize, l: usize) -> bool {
    if x > s.len() / l {
        return false;
    }
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if z_function(s[i..]).first() >= &x {
            count += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    count >= l - 1
}