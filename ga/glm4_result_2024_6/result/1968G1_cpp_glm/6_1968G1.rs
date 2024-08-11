use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for i in 1..n {
        if i <= z[i - 1] {
            z[i] = z[i - 1];
        }
        while i + z[i] < n && &s[z[i]..z[i] + i] == &s[i..i + z[i]] {
            z[i] += 1;
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
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let l: usize = line.next().unwrap().parse().unwrap();
        let r: usize = line.next().unwrap().parse().unwrap();
        let s = line.next().unwrap();

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
        if x <= s.len() - i && &s[i..i + x] == &s[i + x..i + 2 * x] {
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