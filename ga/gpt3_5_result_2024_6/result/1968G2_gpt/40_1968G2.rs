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

fn build(aa: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;

    while n_ < n {
        n_ <<= 1;
    }

    for i in 0..n {
        st[n_ + i] = aa[i];
    }

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }

    st
}

fn query(st: &Vec<i32>, n_: usize, l: usize, a: i32) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut temp_l = l;
                while temp_l < n_ {
                    temp_l = if st[temp_l << 1] >= a { temp_l << 1 } else { (temp_l << 1) + 1 };
                }
                return temp_l - n_;
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
    let mut input = stdin.lock().lines().map(|line| line.unwrap());
    let t: i32 = input.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line: Vec<i32> = input.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = line[0] as usize;
        let kl = line[1] as usize;
        let kr = line[2] as usize;
        let cc = input.next().unwrap().chars().collect::<Vec<char>>();

        let mut zz = vec![0; n];
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

        let st = build(&zz, n);
        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(&st, n_, i + p, p);
                if i < n_ {
                    k += 1;
                }
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