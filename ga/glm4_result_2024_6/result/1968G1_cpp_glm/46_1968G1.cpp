use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    while t > 0 {
        let mut input = lines.next().unwrap().unwrap();
        let n: i32 = input.split_whitespace().next().unwrap().parse().unwrap();
        let l: i32 = input.split_whitespace().nth(1).unwrap().parse().unwrap();
        let r: i32 = input.split_whitespace().nth(2).unwrap().parse().unwrap();
        let s = lines.next().unwrap().unwrap();

        let mut left = 0;
        let mut right = n as usize + 1;
        let z = z_function(&s);

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if is_valid(mid as usize, &z, n, l) {
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

    let mut left = 0;
    let mut right = 0;

    for i in 1..n {
        if i <= right && z[i - left] < right - i + 1 {
            z[i] = z[i - left];
        } else {
            z[i] = std::cmp::max(0, right - i + 1);
            while i + z[i] < n && s.chars().nth(z[i]) == s.chars().nth(i + z[i]) {
                z[i] += 1;
            }
        }

        if i + z[i] - 1 > right {
            left = i;
            right = i + z[i] - 1;
        }
    }

    z
}

fn is_valid(x: usize, z: &Vec<usize>, n: i32, l: i32) -> bool {
    if x as i32 > n / l {
        return false;
    }

    let mut count = 0;
    let mut i = x;

    while i < z.len() {
        if z[i] >= x {
            count += 1;
            i += x;
        } else {
            i += 1;
        }
    }

    count >= l - 1
}