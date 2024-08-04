use std::io;
use std::io::{Read, Write};

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
    for (i, &a) in aa.iter().enumerate() {
        st[n_ + i] = a;
    }
    for i in (1..n_).rev() {
        st[i] = max(st[2 * i], st[2 * i + 1]);
    }
}

fn query(st: &[usize], l: usize, a: usize) -> usize {
    let mut l = l + N_;
    let mut r = N_ - 1;
    while l <= r {
        if l % 2 == 1 {
            if st[l] >= a {
                while l < N_ {
                    l = if st[2 * l] >= a { 2 * l } else { 2 * l + 1 };
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    while t > 0 {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc: String = parts.next().unwrap().to_string();
        let mut zz = vec![0; n];
        let mut st = vec![0; N_ * 2];

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
        build(&zz, n, &mut st);

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