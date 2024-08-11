use std::io::{self, BufRead};

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &mut Vec<i32>, n: usize, st: &mut Vec<i32>) {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    
    st.clear();
    st.resize(2 * n_, 0);

    for i in 0..n {
        st[n_ + i] = aa[i];
    }

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) | 1]);
    }
}

fn query(l: usize, a: i32, st: &Vec<i32>, n_: usize) -> usize {
    let mut r = n_ - 1;
    let mut l = l;

    let mut l = l + n_;
    let mut r = r + n_;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut l_ = l;
                while l_ < n_ {
                    l_ = if st[l_ << 1] >= a { l_ << 1 } else { (l_ << 1) | 1 };
                }
                return l_ - n_;
            }
            l += 1;
        }
    }

    n_
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut input_line = String::new();
    handle.read_line(&mut input_line).unwrap();
    let t: i32 = input_line.trim().parse().unwrap();

    for _ in 0..t {
        input_line.clear();
        handle.read_line(&mut input_line).unwrap();
        let parts: Vec<i32> = input_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let n = parts[0] as usize;
        let kl = parts[1] as i32;
        let kr = parts[2] as i32;
        let cc = parts[3].trim().to_string();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                let mut r = r;
                while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = vec![];
        build(&mut zz, n, &mut st);

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(i + p, p as i32, &st, n);
                if i < n {
                    k += 1;
                }
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