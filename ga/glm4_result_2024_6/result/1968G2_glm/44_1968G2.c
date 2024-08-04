use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

fn build(aa: &[i32], n: usize) -> Vec<i32> {
    let mut st = vec![0; 2 * N_];
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    for (i, &a) in aa.iter().enumerate() {
        st[n_ + i] = a;
    }
    for i in (1..n_).rev() {
        st[i] = max(st[2 * i], st[2 * i + 1]);
    }
    st
}

fn query(st: &[i32], n: usize, l: usize, a: i32) -> usize {
    let r = n - 1;
    let mut l = l + n;
    let mut r = r + n;
    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                while l < n {
                    l = if st[2 * l] >= a { 2 * l } else { 2 * l + 1 };
                }
                return l - n;
            }
            l += 1;
        } else {
            l >>= 1;
            r >>= 1;
        }
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    while t > 0 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let kl = parts.next().unwrap().parse::<i32>().unwrap();
        let kr = parts.next().unwrap().parse::<i32>().unwrap();
        let cc = parts.next().unwrap();
        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = r.max(i);
                while r < n && &cc[r..] == &cc[r - l..] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }
        let st = build(&zz, n);
        let mut ans = vec![0; n + 1];
        for p in 1..=n as i32 {
            let mut k = 0;
            let mut l = 0;
            while l < n {
                let r = query(&st, n, l, p);
                k += (r - l) as i32 / p;
                l = r;
            }
            ans[k as usize] = max(ans[k as usize], p);
        }
        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }
        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
        t -= 1;
    }
}