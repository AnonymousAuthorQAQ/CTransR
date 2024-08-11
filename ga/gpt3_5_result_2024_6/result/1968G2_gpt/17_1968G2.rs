use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &mut [i32], n: usize) -> Vec<i32> {
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

fn query(st: &Vec<i32>, l: usize, a: i32, n_: usize) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut ll = l;
                while ll < n_ {
                    if st[ll << 1] >= a {
                        ll = ll << 1;
                    } else {
                        ll = ll << 1 | 1;
                    }
                }
                return ll - n_;
            }
            l += 1;
        }
        l >>= 1;
    }

    n_
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = iterator.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input_line: Vec<i32> = iterator
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = input_line[0] as usize;
        let kl = input_line[1] as usize;
        let kr = input_line[2] as usize;
        let cc = iterator.next().unwrap();

        let mut zz = vec![0; n];
        let cc_chars: Vec<_> = cc.chars().collect();

        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = r.max(i);
                while r < n && cc_chars[r] == cc_chars[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let st = build(&mut zz, n);
        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(&st, i + p, p as i32, n_);
                if i < n {
                    k += 1;
                }
            }
            ans[k] = ans[k].max(p as i32);
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