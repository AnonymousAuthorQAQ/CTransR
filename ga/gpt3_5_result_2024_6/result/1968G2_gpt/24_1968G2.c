use std::cmp;

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
    for i in (1..n_).rev() {
        st[i] = cmp::max(st[i << 1], st[i << 1 | 1]);
    }
}

fn query(l: usize, a: i32, n_: usize, st: &[i32]) -> usize {
    let mut l = l;
    let mut r = n_ - 1;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut l_val = l;
                while l_val < n_ {
                    l_val = if st[l_val << 1] >= a { l_val << 1 } else { l_val << 1 | 1 };
                }
                return l_val - n_;
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
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().to_string();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        let cc_chars: Vec<char> = cc.chars().collect();
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n && cc_chars[r] == cc_chars[r - l] {
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
            ans[k] = cmp::max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}