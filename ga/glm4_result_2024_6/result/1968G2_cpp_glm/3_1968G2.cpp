use std::io::{self, BufRead};
use std::str::FromStr;

const N: usize = 2e5 as usize + 3;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut n = 0;
    let mut x = 0;
    let mut y = 0;
    let mut a = vec![0; N];
    let mut z = vec![0; N];
    let mut p = vec![0; N];
    let mut t = vec![0; N];
    let mut ans = vec![0; N];

    let mut T = 0;
    if let Ok(T_str) = lines.next() {
        T = T_str.unwrap().parse::<usize>().unwrap();
    }

    for _ in 0..T {
        if let Ok(line) = lines.next() {
            let mut parts = line.split_whitespace();
            n = parts.next().unwrap().parse::<usize>().unwrap();
            x = parts.next().unwrap().parse::<usize>().unwrap();
            y = parts.next().unwrap().parse::<usize>().unwrap();
            a[1..n + 1].copy_from_slice(parts.next().unwrap().as_bytes());
        }

        for i in 1..=n + 1 {
            p[i] = i;
            t[i] = i;
            ans[i] = 0;
            z[i] = 0;
        }
        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t[1..=n].sort_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                c += 1;
                while p[j] != j {
                    p[j] = p[p[j]];
                    j = p[j];
                }
            }
            ans[c] = i;
        }

        for i in (0..n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}