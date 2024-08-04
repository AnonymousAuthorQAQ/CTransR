use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; /* N_ = pow2(ceil(log2(N))) */

fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &mut [usize], n: usize, st: &mut [usize]) {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    st.iter_mut().for_each(|x| *x = 0);
    aa.iter().enumerate().for_each(|(i, &val)| st[n_ + i] = val);
    
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
}

fn query(l: usize, a: usize, n: usize, n_: usize, st: &mut [usize]) -> usize {
    let mut l_val = l + n_;
    let mut r_val = n_ - 1 + n_;

    while l_val <= r_val {
        if l_val & 1 == 1 {
            if st[l_val] >= a {
                let mut idx = l_val;

                while idx < n_ {
                    idx = if st[idx << 1] >= a { idx << 1 } else { idx << 1 | 1 };
                }
                return idx - n_;
            }
            l_val += 1;
        }
        l_val >>= 1;
        r_val >>= 1;
    }
    return n_;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();

        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc: String = parts.next().unwrap().parse().unwrap();

        let mut ans = vec![0; n + 1];
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

        let mut st = vec![0; N_ * 2];
        build(&mut zz, n, &mut st);

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            loop {
                let idx = query(i + p, p, n, N_, &mut st);
                if idx == n_ {
                    break;
                }
                k += 1;
                i = idx;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}