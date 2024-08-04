use std::io::{self, BufRead};
use std::cmp::min;

const N: usize = 2e5 as usize + 3;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut n: usize;
    let mut x: usize;
    let mut y: usize;
    let mut a = vec![0; N];
    let mut z = vec![0; N];
    let mut p = vec![0; N];
    let mut t = vec![0; N];
    let mut ans = vec![0; N];

    let mut line = lines.next().unwrap().unwrap();
    let t = line.parse::<usize>().unwrap();

    for _ in 0..t {
        line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        n = iter.next().unwrap().parse::<usize>().unwrap();
        x = iter.next().unwrap().parse::<usize>().unwrap();
        y = iter.next().unwrap().parse::<usize>().unwrap();
        a[1..n + 1].copy_from_slice(iter.next().unwrap().as_bytes());

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
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t[1..n + 1].sort_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 1;
            let mut c = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            for j in 1..=n {
                c += 1;
                while j + i <= n {
                    j = p[j + i];
                }
            }
            ans[c] = i;
        }

        for i in (0..n).rev() {
            ans[i] = max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}