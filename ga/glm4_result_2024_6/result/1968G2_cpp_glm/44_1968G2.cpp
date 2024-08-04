const N: usize = 2e5 as usize + 3;
use std::io::{self, BufRead};

fn find(k: usize) -> usize {
    if k > N || p[k] == k {
        return k;
    }
    p[k] = find(p[k]);
    p[k]
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    let mut t = 1;

    while t > 0 {
        let _ = stdin.read_line(&mut line);
        let mut v = line.trim().split_whitespace().map(|s| s.parse().unwrap());
        t = v.next().unwrap();

        for _ in 0..t {
            let (n, x, y) = (v.next().unwrap(), v.next().unwrap(), v.next().unwrap());
            line.clear();
            stdin.read_line(&mut line);
            let a = line.trim().as_bytes();

            for i in 1..=n {
                p[i] = i;
                t[i] = i;
                ans[i] = 0;
                z[i] = 0;
            }

            z[1] = n;

            for i in 2..=n {
                z[i] = std::cmp::min(z[i - z[1] + 1], z[1] + z[1] - i);
                while a[i + z[i]] == a[1 + z[i]] {
                    z[i] += 1;
                }
            }

            t.sort_by(|&i, &j| z[i].cmp(&z[j]));

            for i in 1..=n {
                let mut c = 0;
                while k <= n && z[t[k]] < i {
                    p[t[k]] = t[k] + 1;
                    k += 1;
                }
                for j in 1..=n {
                    c += 1;
                }
                ans[c] = i;
            }

            for i in (1..=n).rev() {
                ans[i] = std::cmp::max(ans[i], ans[i + 1]);
            }

            for i in x..=y {
                print!("{} ", ans[i]);
            }
            println!();
        }
    }
}