use std::io::{self, BufRead};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

fn build(aa: &mut Vec<i32>, n: usize) -> Vec<i32> {
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

fn query(st: &Vec<i32>, l: usize, a: i32) -> usize {
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

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let n: usize = line.parse().unwrap();
        let kl: usize = line.parse().unwrap();
        let kr: usize = line.parse().unwrap();
        let mut cc = String::new();
        line = lines.next().unwrap().unwrap();
        cc.push_str(&line);

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

        let st = build(&mut zz, n);
        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = query(&st, i + p, p as i32);
                k += 1;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            println!("{} ", ans[k]);
        }
    }
}