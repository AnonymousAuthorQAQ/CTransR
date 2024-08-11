use std::io;
use std::cmp::{min, max};
use std::collections::HashMap;

const N: usize = 2e5 as usize + 3;
const MAX: i32 = i32::MAX;

fn find(p: &mut Vec<i32>, mut k: i32) -> i32 {
    if k > N as i32 || p[k as usize] == k {
        k
    } else {
        p[k as usize] = find(p, p[k as usize]);
        p[k as usize]
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let t: i32 = line.trim().parse().expect("Failed to parse T");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut parts = line.trim().split_whitespace();
        let n: i32 = parts.next().expect("Failed to read n").parse().expect("Failed to parse n");
        let x: i32 = parts.next().expect("Failed to read x").parse().expect("Failed to parse x");
        let y: i32 = parts.next().expect("Failed to read y").parse().expect("Failed to parse y");
        let s = parts.next().expect("Failed to read s");
        let mut a = vec![0; N];
        let mut z = vec![0; N];
        let mut p = vec![0; N];
        let mut t = vec![0; N];
        let mut ans = vec![0; N];

        for (i, c) in s.chars().enumerate() {
            a[i + 1] = c as i32;
        }

        z[1] = n;
        let mut j = 0;
        for i in 2..=n as usize {
            if j + z[j] > i as i32 {
                z[i] = min(z[i - j + 1], j + z[j] - i as i32);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i as i32 + z[i] > j as i32 + z[j] {
                j = i;
            }
        }

        for i in 1..=n as usize {
            t[i] = i;
        }

        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n as usize {
            let mut c = 0;
            while find(&mut p, t[i]) <= n as i32 {
                c += 1;
            }
            ans[c] = i as i32;
        }

        for i in n as usize..1 {
            ans[i] = max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i as usize]);
        }
        println!("");
    }
}