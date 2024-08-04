use std::io;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;

    for (i, c) in s.chars().enumerate() {
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

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut numbers = input.trim().split_whitespace();
        let n: i32 = numbers.next().unwrap().parse().unwrap();
        let l: i32 = numbers.next().unwrap().parse().unwrap();
        let r: i32 = numbers.next().unwrap().parse().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        let mut left = 0;
        let mut right = n;

        while left < right {
            let mid = (left + right) / 2;
            if is_valid(mid, s.as_bytes(), l, r) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        println!("{}", left - 1);
    }
}

fn is_valid(x: i32, s: &[u8], l: i32, r: i32) -> bool {
    if x as usize > s.len() / (l - 1) as usize {
        return false;
    }

    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if i + x as usize <= s.len() && &s[i..i + x as usize] == &s[i..i + x as usize].repeat(l as usize - 1) {
            count += 1;
            i += x as usize;
        } else {
            i += 1;
        }
    }

    count >= l - 1
}