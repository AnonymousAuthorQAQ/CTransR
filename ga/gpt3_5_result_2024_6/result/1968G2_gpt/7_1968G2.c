use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;

    while n_ < n {
        n_ <<= 1;
    }

    for i in 0..n {
        st[n_ + i] = aa[i];
    }

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1 | 0], st[i << 1 | 1]);
    }

    st
}

fn query(l: usize, a: i32, n_: usize, st: &Vec<i32>) -> usize {
    let mut r = n_ - 1;
    let mut l = l + n_;

    for r += n_; l <= r; l >>= 1 {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut l = l;
                while l < n_ {
                    l = if st[l << 1 | 0] >= a { l << 1 | 0 } else { l << 1 | 1 };
                }
                return l - n_;
            }
            l += 1;
        }
    }

    n_
}

fn main() {
    let stdin = io::stdin();
    
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let kl = parts[1];
        let kr = parts[2];

        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let cc: Vec<char> = input.trim().chars().collect();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = r.max(i);
                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let st = build(&zz, n);
        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(i + p, p as i32, n_, &st);
                if i != n_ {
                    k += 1;
                }
            }

            ans[k] = ans[k].max(p);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = ans[k].max(ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}