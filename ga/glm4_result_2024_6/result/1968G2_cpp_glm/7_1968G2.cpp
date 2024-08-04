use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

const N: usize = 2e5 as usize + 3;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut stdout = io::stdout();

    let mut n: usize;
    let mut x: usize;
    let mut y: usize;
    let mut a = vec![0; N];
    let mut z = vec![0; N];
    let mut p = vec![0; N];
    let mut t = vec![0; N];
    let mut ans = vec![0; N];

    let mut line = lines.next().unwrap().unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line = lines.next().unwrap().unwrap();
        let mut iter = line.trim().split_whitespace();
        n = iter.next().unwrap().parse().unwrap();
        x = iter.next().unwrap().parse().unwrap();
        y = iter.next().unwrap().parse().unwrap();
        let s: String = iter.next().unwrap().to_string();
        a[1..=n].copy_from_slice(s.as_bytes());

        for i in 1..=n + 1 {
            p[i] = i;
            t[i] = i;
            ans[i] = 0;
            z[i] = 0;
        }
        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = z[i - j + 1].min(j + z[j] - i);
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
                let mut j = Find(j + i);
                c += 1;
            }
            ans[c] = i;
        }

        for i in (0..n).rev() {
            ans[i] = ans[i + 1].max(ans[i]);
        }

        for i in x..=y {
            write!(stdout, "{} ", ans[i]).unwrap();
        }
        writeln!(stdout).unwrap();
    }
}

fn Find(k: usize) -> usize {
    if k > n || p[k] == k {
        k
    } else {
        p[k] = Find(p[k]);
        p[k]
    }
}