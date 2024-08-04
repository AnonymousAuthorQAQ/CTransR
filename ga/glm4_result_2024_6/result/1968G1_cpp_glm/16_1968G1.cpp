use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.chars().enumerate() {
        if i > 0 && z[i - 1] < n - i {
            let mut j = i + z[i - 1];
            while j < n && s.chars().nth(j) == Some(c) {
                j += 1;
            }
            z[i] = j - i;
        }
    }
    z
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let l = parts.next().unwrap().parse::<usize>().unwrap();
        let r = parts.next().unwrap().parse::<usize>().unwrap();
        let s = lines.next().unwrap();
        let mut left = 0;
        let mut right = n + 1;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if can_divide(s, mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left);
    }
}

fn can_divide(s: &str, x: usize, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if x <= s.len() - i && z_function(&s[i..i + x]) == vec![x; x] {
            count += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    count >= l - 1
}