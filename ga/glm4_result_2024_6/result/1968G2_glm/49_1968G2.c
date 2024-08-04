use std::io::{self, BufRead};

const N: usize = 200000;
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
    let mut l = l + N_;
    let mut r = N_ - 1;
    while l <= r {
        if (l & 1) == 1 {
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
        let mut cc = line.split_whitespace().next().unwrap().to_string();
        let mut zz = vec![0; n];

        for i in 1..n {
            if i + zz[i - 1] < zz[i] {
                zz[i] = zz[i - 1];
            } else {
                let mut r = i;
                while r < n && &cc[r..] == &cc[i..i + (r - i)] {
                    r += 1;
                }
                zz[i] = r - i;
            }
        }

        let st = build(&zz, n);
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