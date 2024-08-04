use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18; /* N_ = pow2(ceil(log2(N))) */

fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &mut [usize], n: usize) {
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
}

fn query(l: usize, a: usize, n_: usize, st: &Vec<usize>) -> usize {
    let mut r = n_ - 1;
    let mut l = l;
    let mut r = r;

    while l <= r {
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
        let cc = iter.next().unwrap();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        let cc_chars: Vec<char> = cc.chars().collect();

        for i in 1..n {
            let mut l = i;
            let mut r = i;

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

        build(&mut zz, n);

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(i + p, p, n, &zz);

                if i < n {
                    k += 1;
                }
            }

            ans[k] = cmp::max(ans[k], p);
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