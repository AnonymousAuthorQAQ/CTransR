use std::io::{self, Read, Write};
use std::str::FromStr;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            z[i] = n;
        } else {
            let mut l = i;
            let mut r = i;
            while r < n && s.chars().nth(r) == s.chars().nth(l) {
                r += 1;
            }
            z[i] = r - i;
            if i + z[i] > r {
                l = i;
                r = i + z[i];
            }
        }
    }
    z
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if can_repeat(mid, s, l) {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left);
    }
}

fn can_repeat(x: usize, s: &str, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if x <= s.len() - i && z_function(&s[i..]).first() >= &x {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_function() {
        let s = "abcabcabcabc";
        let z = z_function(s);
        assert_eq!(z, vec![12, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_can_repeat() {
        let s = "abcabcabcabc";
        assert!(can_repeat(3, s, 3));
        assert!(!can_repeat(4, s, 3));
    }
}