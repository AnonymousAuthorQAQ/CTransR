use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = 1 << 18; /* N_ = pow2(ceil(log2(N))) */

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn build(aa: &mut [usize], n: usize, st: &mut [usize]) {
    let mut n_ = 1;

    while n_ < n {
        n_ <<= 1;
    }

    st.iter_mut().for_each(|x| *x = 0);
    
    for i in 0..n {
        st[n_ + i] = aa[i];
    }

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }
}

fn query(l: usize, a: usize, n_: usize, st: &[usize]) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1;

    for i in l..=r {
        let mut i = i;
        let mut r = r;

        while i > 0 {
            if i & 1 == 1 {
                if st[i] >= a {
                    while i < n_ {
                        i = if st[i << 1] >= a { i << 1 } else { (i << 1) + 1 };
                    }
                    return i - n_;
                }
                i += 1;
            }
            i >>= 1;
            r >>= 1;
        }
    }

    n_
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.trim().split(" ");
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc: String = parts.next().unwrap().parse().unwrap();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n+1];
        
        let mut l = 0;
        let mut r = 0;
        
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);

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

            let mut query_result = query(i + p, p, N_, &st);

            while query_result < n {
                k += 1;
                i = query_result;
                query_result = query(i + p, p, N_, &st);
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