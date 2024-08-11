use std::io;
use std::io::{BufRead, Write};

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

fn build(aa: &[i32], n: usize) -> Vec<i32> {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    let mut st = vec![0; n_ * 2];
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[i * 2], st[i * 2 + 1]);
    }
    st
}

fn query(st: &[i32], l: usize, r: usize, a: i32) -> usize {
    let mut l = l + r;
    let mut r = r;
    while l < r {
        if l & 1 == 1 {
            if st[l] >= a {
                while l < r {
                    l = if st[l * 2] >= a { l * 2 } else { l * 2 + 1 };
                }
                return l - r;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    r
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut tokens = line.trim().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let kl: usize = tokens.next().unwrap().parse().unwrap();
        let kr: usize = tokens.next().unwrap().parse().unwrap();
        let cc: String = tokens.next().unwrap().to_string();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc.chars().nth(r).unwrap() == cc.chars().nth(r - l).unwrap() {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let st = build(&zz, n);
        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = query(&st, i, n, p as i32);
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            stdout.write_fmt!("{} ", ans[k]).unwrap();
        }
        writeln!(stdout).unwrap();
    }
}