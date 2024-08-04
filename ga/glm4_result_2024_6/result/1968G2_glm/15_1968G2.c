use std::io::{self, Read};

fn build(aa: &[i32], n: usize) -> Vec<i32> {
    let mut st = vec![0; n * 4];
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = st[i << 1].max(st[i << 1 | 1]);
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc: Vec<char> = parts.next().unwrap().chars().collect();

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
                i = query(&st, i, n - 1, p as i32);
                k += 1;
            }
            ans[k] = ans[k].max(p);
        }

        for k in (1..n).rev() {
            ans[k] = ans[k].max(ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}