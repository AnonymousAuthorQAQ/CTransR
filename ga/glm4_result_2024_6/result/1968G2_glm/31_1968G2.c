const N: usize = 200000;
const N_: usize = (1 << 18); // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    a.max(b)
}

fn build(aa: &mut Vec<usize>, n: usize) -> Vec<usize> {
    let mut st = vec![0; n << 1];
    for i in 0..n {
        st[n + i] = aa[i];
    }
    for i in (1..n).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
    st
}

fn query(st: &Vec<usize>, l: usize, a: usize) -> usize {
    let r = st.len() - 1;
    let mut l = l + N_;
    let mut r = r + N_;
    while l <= r {
        if (l & 1) == 1 {
            if st[l] >= a {
                while l < N_ {
                    l = if st[l << 1] >= a { l << 1 } else { l << 1 | 1 };
                }
                return l - N_;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    return N_;
}

fn main() {
    let mut t = 0;
    let mut cc = String::new();
    let mut ans = vec![0; N + 1];

    loop {
        t += 1;
        if t > 1 {
            break;
        }

        let mut n = 0;
        let mut kl = 0;
        let mut kr = 0;

        let mut zz = vec![0; N];
        let mut i = 1;
        let mut l = 0;
        let mut r = 0;

        while i < n {
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
            i += 1;
        }

        let st = build(&mut zz, n);
        let mut p = 1;

        while p <= n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(&st, i + p, p);
                k += 1;
            }

            ans[k] = max(ans[k], p);
            p += 1;
        }

        let mut k = n - 1;

        while k > 0 {
            ans[k] = max(ans[k], ans[k + 1]);
            k -= 1;
        }

        let mut k = kl;

        while k <= kr {
            println!("{} ", ans[k]);
            k += 1;
        }

        println!();
    }
}