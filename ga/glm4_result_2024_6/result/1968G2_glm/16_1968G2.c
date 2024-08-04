use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    a.max(b)
}

fn build(aa: &[usize], n: usize, st: &mut [usize]) {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    st[..n_].fill(0);
    for (i, &a) in aa.iter().enumerate() {
        st[n_ + i] = a;
    }
    for i in (1..n_).rev() {
        st[i] = max(st[2 * i], st[2 * i + 1]);
    }
}

fn query(l: usize, a: usize, st: &[usize]) -> usize {
    let mut l = l + N_;
    let mut r = N_ - 1;
    while l <= r {
        if l & 1 == 1 {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let n: usize = line.parse().unwrap();
        let kl: usize = line.parse().unwrap();
        let kr: usize = line.parse().unwrap();
        let mut cc: Vec<char> = line.chars().collect();

        let mut zz = vec![0; n];
        let mut st = vec![0; N_ * 2];

        for i in 1..n {
            if i + zz[i - 1] < zz.len() {
                zz[i] = zz[i - 1];
            } else {
                let mut l = i;
                let mut r = i;
                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        build(&zz, n, &mut st);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut l = 0;
            while l < n {
                let r = query(l, p, &st);
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