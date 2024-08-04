use std::io::{self, BufRead};
use std::cmp::Ordering;

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

    for _ in 0.. {
        if let Ok(line) = lines.next() {
            let mut iter = line.split_whitespace();
            if let (Some(n_str), Some(x_str), Some(y_str), Some(a_str)) = (iter.next(), iter.next(), iter.next(), iter.next()) {
                n = n_str.parse().unwrap();
                x = x_str.parse().unwrap();
                y = y_str.parse().unwrap();
                a[1..=n].copy_from_slice(a_str.as_bytes());
            }
        } else {
            break;
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
            while i + z[i] > j + z[j] && a[i + z[i]] == a[1 + z[i]] {
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
                if let Some(pj) = p.get_mut(j + i) {
                    if *pj == j + i {
                        c += 1;
                    }
                }
            }
            ans[c] = i;
        }

        for i in (0..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}