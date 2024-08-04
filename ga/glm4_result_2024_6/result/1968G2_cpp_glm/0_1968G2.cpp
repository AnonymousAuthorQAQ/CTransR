use std::io::{self, BufRead};
use std::cmp::Ordering;

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

    let mut T: usize;
    lines.next().unwrap().parse::<usize>().map(|t| T = t).unwrap();

    while T > 0 {
        lines.next().unwrap().split_whitespace().map(|s| s.parse::<usize>()).zip(&mut [&mut n, &mut x, &mut y]).for_each(|(s, p)| *p = s.unwrap());
        lines.next().unwrap().chars().enumerate().for_each(|(i, c)| a[i + 1] = c as i32);

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
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }

            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }

            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_unstable_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 1;
            let mut c = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }

            for j in 1..=n {
                c += 1;
                while p[j] > j + i {
                    j = p[j];
                }
            }

            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
        T -= 1;
    }
}