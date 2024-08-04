use std::io;
use std::io::{stdout, Write};

const N: usize = 2e5 as usize + 3;
const T: usize = 1;

fn main() {
    let mut stdout = stdout();
    let n: usize;
    let x: usize;
    let y: usize;
    let mut a = String::new();
    let mut z = vec![0; N];
    let mut p = vec![0; N];
    let mut t = vec![0; N];
    let mut ans = vec![0; N];

    for _ in 0..T {
        io::stdin().read_line(&mut a).unwrap();
        let a: Vec<char> = a.trim().chars().collect();
        let n = a.len();
        for i in 1..=n {
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

        t.sort_by(|i, j| z[*i] < z[*j]);

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
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            write!(stdout, "{} ", ans[i]).unwrap();
        }
        write!(stdout, "\n").unwrap();
    }
}

fn Find(k: usize) -> usize {
    if k > N || p[k] == k {
        return k;
    }
    p[k] = Find(p[k]);
    p[k]
}