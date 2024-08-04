use std::io::{self, BufRead};

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

fn build(aa: &[i32], n: usize) -> Vec<i32> {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    let mut st = vec![0; n_ * 2];
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
    st
}

fn query(st: &[i32], l: usize, r: usize, a: i32) -> usize {
    let mut l = l + st.len() / 2;
    let mut r = r + st.len() / 2;
    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                while l < st.len() / 2 {
                    l = if st[l << 1] >= a { l << 1 } else { l << 1 | 1 };
                }
                return l - st.len() / 2;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    st.len() / 2
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

        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc.chars().nth(r - l) == cc.chars().nth(r) {
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
                i = query(&st, i, n - 1, p);
                k += 1;
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