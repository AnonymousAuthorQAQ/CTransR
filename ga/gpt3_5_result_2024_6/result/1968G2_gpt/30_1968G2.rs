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

fn build(aa: &[usize], n: usize) -> Vec<usize> {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;

    while n_ < n {
        n_ <<= 1;
    }

    for i in 0..n {
        st[n_ + i] = aa[i];
    }

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }

    st
}

fn query(l: usize, a: usize, n_: usize, st: &Vec<usize>) -> usize {
    let mut l = l;
    let mut r = n_ - 1;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut temp_l = l;
                while temp_l < n_ {
                    temp_l = if st[temp_l << 1] >= a {
                        temp_l << 1
                    } else {
                        temp_l << 1 | 1
                    };
                }

                return temp_l - n_;
            }

            l += 1;
        }

        l >>= 1;
        r >>= 1;
    }

    return n_;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().parse().unwrap();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);

                while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
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
                i = query(i + p, p, n, &st);
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