use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, char) in s.chars().enumerate().skip(1) {
        let mut l = 0;
        let mut r = 0;
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = std::cmp::max(0, r - i + 1);
            while i + z[i] < n && &s[z[i]..z[i] + 1] == &s[i..i + z[i] + 1] {
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

fn find_repeating_substring(s: &str, l: usize) -> usize {
    let mut left = 0;
    let mut right = s.len() + 1;
    while left + 1 < right {
        let mid = (left + right) / 2;
        if can_repeat(s, mid, l) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn can_repeat(s: &str, x: usize, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if x <= s.len() / l {
            count += 1;
            i += x;
        } else {
            break;
        }
    }
    count >= l - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        let mut iter = input.lines();
        let n: usize = iter.next().unwrap().trim().parse().unwrap();
        let l: usize = iter.next().unwrap().trim().parse().unwrap();
        let r: usize = iter.next().unwrap().trim().parse().unwrap();
        let s = iter.next().unwrap().trim();
        let result = find_repeating_substring(s, l);
        println!("{}", result);
    }
}