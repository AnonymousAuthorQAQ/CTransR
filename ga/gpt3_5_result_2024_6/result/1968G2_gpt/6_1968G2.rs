use std::cmp;
use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn build(aa: &mut [i32], n: usize, st: &mut [i32]) {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    st.iter_mut().for_each(|x| *x = 0);
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
}

fn query(l: usize, a: i32, st: &[i32], n_: usize) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut tmp = l;
                while tmp < n_ {
                    tmp = if st[tmp << 1] >= a { tmp << 1 } else { tmp << 1 | 1 };
                }
                return tmp - n_;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    n_
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc = parts.next().unwrap();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = vec![0; N_ * 2];
        build(&mut zz, n, &mut st);
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            loop {
                i = query(i + p, p as i32, &st, N_);
                if i == n_ {
                    break;
                }
                k += 1;
            }
            ans[k] = cmp::max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}