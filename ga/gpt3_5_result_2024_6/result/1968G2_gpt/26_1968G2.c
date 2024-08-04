use std::io::{self, Read};

const N: usize = 200000;
const N_: usize = 1 << 18; /* N_ = pow2(ceil(log2(N))) */

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
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
    let mut i = n_ as i32 - 1;
    while i > 0 {
        st[i as usize] = max(st[(i << 1) as usize], st[(i << 1 | 1) as usize]);
        i -= 1;
    }
}

fn query(l: usize, a: i32, n_: usize, st: &[i32]) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1 + n_;
    while l <= r {
        if (l & 1) == 1 {
            if st[l] >= a {
                let mut tmp_l = l;
                while tmp_l < n_ {
                    tmp_l = if st[(tmp_l << 1) as usize] >= a {
                        (tmp_l << 1) | 0
                    } else {
                        (tmp_l << 1) | 1
                    };
                }
                return tmp_l - n_;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    n_
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().lines();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = line[0];
        let kl = line[1];
        let kr = line[2];
        let cc = lines.next().unwrap().trim();
        
        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = r.max(i);
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
            while i < n {
                i = query(i + p, p as i32, N_, &st);
                if i < n {
                    k += 1;
                }
            }
            ans[k] = ans[k].max(p as i32);
        }

        for k in (n - 1)..0 {
            ans[k] = ans[k].max(ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}