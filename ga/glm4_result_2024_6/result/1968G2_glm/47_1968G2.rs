use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    a.max(b)
}

fn build(aa: &[usize], n: usize) -> Vec<usize> {
    let mut st = vec![0; n << 1];
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

fn query(st: &mut [usize], l: usize, r: usize, a: usize) -> usize {
    let mut l = l + N_;
    let mut r = r + N_;
    while l < r {
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
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = parts[0];
        let kl = parts[1];
        let kr = parts[2];
        let cc = parts[3];

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && &cc[r..r+l] == &cc[l..l+r] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = build(&zz, n);
        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                let idx = query(&mut st, i, n - 1, p);
                if idx == N_ {
                    break;
                }
                k += 1;
                i = idx + p;
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