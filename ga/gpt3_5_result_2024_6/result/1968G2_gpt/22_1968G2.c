use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = 2^ceil(log2(N))

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn build(aa: &Vec<usize>, n: usize) -> (Vec<usize>, usize) {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;

    while n_ < n {
        n_ <<= 1;
    }

    st[n_..(n_ + n)].clone_from_slice(&aa);
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }

    (st, n_)
}

fn query(st: &Vec<usize>, n_: usize, mut l: usize, a: usize) -> usize {
    let mut r = n_ - 1;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                while l < n_ {
                    l = if st[l * 2] >= a { l * 2 } else { l * 2 + 1 };
                }
                return l - n_;
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
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc: String = parts.next().unwrap().parse().unwrap();

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

        let (st, n_) = build(&zz, n);
        
        let mut ans = vec![0; n+1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                let idx = query(&st, n_, i + p, p);
                if idx == n_ {
                    break;
                }
                k += 1;
                i = idx;
            }

            ans[k] = max(ans[k], p);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}