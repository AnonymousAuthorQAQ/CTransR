use std::io;
use std::io::prelude::*;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    a.max(b)
}

fn build(aa: &[usize], n: usize, st: &mut [usize]) {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    st[..n_ * 2].fill(0);
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
}

fn query(st: &[usize], l: usize, a: usize) -> usize {
    let mut l = l + N_;
    let mut r = N_ * 2 - 1;
    while l <= r {
        if (l & 1) == 1 {
            if st[l] >= a {
                while l < N_ {
                    l = if st[l << 1] >= a { l << 1 } else { l << 1 | 1 };
                }
                return l - N_;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    N_
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let mut parts = buffer.trim().split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let kl = parts.next().unwrap().parse::<usize>().unwrap();
        let kr = parts.next().unwrap().parse::<usize>().unwrap();
        let cc = parts.next().unwrap();

        let mut zz = vec![0; n];
        let mut st = vec![0; N_ * 2];

        for i in 1..n {
            if i + zz[i - 1] < zz.len() {
                zz[i] = zz[i - 1];
            } else {
                let mut l = i;
                let mut r = i;
                while r < n && cc.chars().nth(r - l).unwrap() == cc.chars().nth(r).unwrap() {
                    r += 1;
                }
                zz[i] = r - l;
                l = i;
                r = i;
            }
        }

        build(&zz, n, &mut st);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut l = 0;
            while l < n {
                let r = query(&st, l, p);
                k += (r - l) / p;
                l = r;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}