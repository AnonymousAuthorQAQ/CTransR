use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &Vec<i32>, n: usize) -> (Vec<i32>, usize) {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;

    while n_ < n {
        n_ <<= 1;
    }

    let mut idx = n_;
    for &val in aa {
        st[idx] = val;
        idx += 1;
    }

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }

    (st, n_)
}

fn query(l: usize, a: i32, st: &Vec<i32>, n_: usize) -> usize {
    let mut r = n_ - 1;
    let mut l = l + n_;

    while l <= r {
        if (l & 1) == 1 {
            if st[l] >= a {
                let mut l = l;
                while l < n_ {
                    l = if st[l << 1] >= a {
                        l << 1
                    } else {
                        (l << 1) + 1
                    };
                }
                return l - n_;
            }
            l += 1;
        }
        l >>= 1;
    }

    n_
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: i32 = iter.next().unwrap().parse().unwrap();
        let kr: i32 = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().to_string();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);

                while r < n && cc.chars().nth(r) == cc.chars().nth(r - l) {
                    r += 1;
                }

                zz[i] = r - l;
            }
        }

        let (st, n_) = build(&zz, n);
        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(i + p, p as i32, &st, n_);
                if i < n_ {
                    k += 1;
                }
            }

            ans[k] = cmp::max(ans[k], p);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
    }
}