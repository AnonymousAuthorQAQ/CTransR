use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = 1 << 18;

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn build(aa: &mut [usize], n: usize, st: &mut [usize]) {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    st[..n_].clone_from_slice(aa);
    st[n_..].iter_mut().for_each(|x| *x = 0);

    for i in (0..n_ - 1).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
}

fn query(l: usize, a: usize, st: &[usize], n_: usize) -> usize {
    let mut r = n_ - 1;
    let mut l = l + n_;
    let mut r = r + n_;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut l = l;
                while l < n_ && st[l << 1] >= a {
                    l = if st[l << 1] >= a { l << 1 } else { l << 1 | 1 };
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
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).expect("Failed to read input");
    let t: i32 = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).expect("Failed to read test case data");
        let mut iter = buffer.split_whitespace();

        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: Vec<char> = iter.next().unwrap().chars().collect();

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

                while r < n && cc[r] == cc[r - l] {
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
                i = query(i + p, p, &st, N_);
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