use std::io::{self, BufRead};
use std::str;

const N: usize = 2e5 as usize + 3;
const MAX_N: usize = N + 1;

fn find(p: &mut [usize], k: usize) -> usize {
    if k > N { return k; }
    if p[k] == k { return k; }
    p[k] = find(p, p[k]);
    p[k]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut n = 0;
    let mut x = 0;
    let mut y = 0;
    let mut a = vec![0; MAX_N];
    let mut z = vec![0; MAX_N];
    let mut p = vec![0; MAX_N];
    let mut t = vec![0; MAX_N];
    let mut ans = vec![0; MAX_N];

    let mut T = 0;
    lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..T {
        n = lines.next().unwrap().parse::<usize>().unwrap();
        x = lines.next().unwrap().parse::<usize>().unwrap();
        y = lines.next().unwrap().parse::<usize>().unwrap();
        let s = lines.next().unwrap();
        a[1..n + 1].copy_from_slice(s.as_bytes());

        for i in 1..MAX_N {
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
                while j <= n {
                    j = find(&mut p, j + i);
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
        println!("");
    }
}