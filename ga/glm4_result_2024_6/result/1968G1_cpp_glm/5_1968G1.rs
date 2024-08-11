use std::io::{self, Read};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    for i in 1..n {
        let mut l = i;
        let mut r = i;
        while r < n && s.chars().nth(r).unwrap() == s.chars().nth(i + (r - i)).unwrap() {
            r += 1;
        }
        z[i] = r - i;
        if i + z[i] < n {
            l = i + z[i];
            r = i + z[i];
            while r < n && s.chars().nth(r).unwrap() == s.chars().nth(i + (r - i)).unwrap() {
                r += 1;
            }
            z[i] = r - i;
        }
    }
    z[0] = n;
    z
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n = parts.next().unwrap().parse::<i32>().unwrap();
        let l = parts.next().unwrap().parse::<i32>().unwrap();
        let r = parts.next().unwrap().parse::<i32>().unwrap();
        let s = lines.next().unwrap();

        let mut ls = 0;
        let mut rs = n + 1;

        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            let mut i = mid;
            let mut cnt = 0;
            while i < s.len() {
                if z[i] >= mid {
                    cnt += 1;
                    i += mid;
                } else {
                    i += 1;
                }
            }
            if cnt >= l - 1 {
                ls = mid;
            } else {
                rs = mid;
            }
        }

        println!("{}", ls);
    }
}