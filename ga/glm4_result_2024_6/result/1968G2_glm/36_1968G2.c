use std::io::{self, BufRead};
use std::cmp::max;

const N: usize = 200_000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    a.max(b)
}

fn build(aa: &[usize], n: usize) -> Vec<usize> {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
    st
}

fn query(st: &[usize], l: usize, a: usize) -> usize {
    let r = N_ - 1;
    let mut l = l + N_;
    let mut r = r + N_;
    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                while l < N_ {
                    l = if st[l << 1] >= a { l << 1 } else { l << 1 | 1 };
                }
                return l - N_;
            }
            l += 1;
        } else {
            l >>= 1;
            r >>= 1;
        }
    }
    N_
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    while t > 0 {
        let n: usize;
        let kl: usize;
        let kr: usize;
        let mut cc = String::new();
        lines.next().unwrap().unwrap().split_whitespace().for_each(|s| {
            if let Ok(num) = s.parse::<usize>() {
                if n == 0 {
                    n = num;
                } else if kl == 0 {
                    kl = num;
                } else {
                    kr = num;
                }
            } else {
                cc.push_str(s);
            }
        });

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc.chars().nth(r) == cc.chars().nth(r - l) {
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
                i = query(&st, i + p, p);
                k += 1;
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

        t -= 1;
    }
}