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

    st[n_..n_+n].clone_from_slice(aa);

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) | 1]);
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
                    temp_l = if st[temp_l << 1] >= a { temp_l << 1 } else { temp_l << 1 | 1 };
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
        let line = input.next().unwrap();
        let mut values = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = values.next().unwrap();
        let kl = values.next().unwrap();
        let kr = values.next().unwrap();
        let cc = values.next().unwrap().to_string();

        let zz: Vec<i32> = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            let idx = i - l;
            if i + zz[idx as usize] < r {
                zz[i] = zz[idx as usize];
            } else {
                l = i;
                r = r.max(i);
                while r < n as usize && cc.chars().nth(r).unwrap() == cc.chars().nth(r - l).unwrap() {
                    r += 1;
                }
                zz[i] = (r - l) as i32;
            }
        }

        let st = build(&zz, n as usize);
        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(&st, N_, i + p as usize, p);
                if i < N_ {
                    k += 1;
                }
            }

            ans[k as usize] = ans[k as usize].max(p);
        }

        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
    }
}