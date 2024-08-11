use std::io::{self, BufRead};

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
    st[0..n_].fill(0);
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
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
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    while t > 0 {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc = parts.next().unwrap();

        let mut zz = vec![0; n];
        let mut st = vec![0; N_ * 2];

        for i in 1..n {
            if i + zz[i - 1] < i {
                zz[i] = zz[i - 1];
            } else {
                let mut l = i;
                let mut r = i;
                while r < n && cc.chars().nth(r - l) == Some(cc.chars().nth(r)) {
                    r += 1;
                }
                zz[i] = r - l;
                l = i;
                r = max(r, i);
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