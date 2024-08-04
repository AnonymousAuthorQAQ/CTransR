use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            z[i] = n - i;
        } else {
            let mut l = i;
            let mut r = i;
            if i < z[i - 1] {
                l = i;
                r = z[i - 1] + i - 1;
            }
            while r < n && &s[l..] == &s[r..r + l] {
                l += 1;
                r += 1;
            }
            z[i] = r - i;
        }
    }
    z
}

fn f(s: &str, x: usize, l: usize) -> bool {
    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if x <= s.len() - i && z_function(&s[i..i + x])[0] >= x {
            cnt += 1;
            i += x;
        } else {
            i += 1;
        }
    }
    cnt >= l - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n = line.next().unwrap().parse::<usize>().unwrap();
        let l = line.next().unwrap().parse::<usize>().unwrap();
        let r = line.next().unwrap().parse::<usize>().unwrap();
        let s = line.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;
        while left < right - 1 {
            let mid = left + (right - left) / 2;
            if f(s, mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left);
    }
}