use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; /* N_ = pow2(ceil(log2(N))) */

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
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }

    st
}

fn query(l: usize, a: i32, st: &Vec<i32>, n_: usize) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1 + n_;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut temp = l;
                while temp < n_ {
                    temp = if st[temp << 1] >= a {
                        (temp << 1) | 0
                    } else {
                        (temp << 1) | 1
                    };
                }
                return temp - n_;
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
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input_line: String = lines.next().unwrap();
        let mut parts = input_line.split_whitespace();

        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: i32 = parts.next().unwrap().parse().unwrap();
        let kr: i32 = parts.next().unwrap().parse().unwrap();
        let cc: Vec<char> = parts.next().unwrap().chars().collect();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);

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
                i = query(i + p, p as i32, &st, n_);
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
    }
}