use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = (1 << 18); // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    a.max(b)
}

fn build(aa: &mut [usize], n: usize) -> Vec<usize> {
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
        }
        l >>= 1;
        r >>= 1;
    }
    N_
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().to_string();
        let mut zz: Vec<usize> = vec![0; n];
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
        let st = build(&mut zz, n);
        let mut ans: Vec<usize> = vec![0; n + 1];
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
            println!("{} ", ans[k]);
        }
    }
}