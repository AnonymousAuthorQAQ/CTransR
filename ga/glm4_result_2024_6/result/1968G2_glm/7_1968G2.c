use std::io::{self, Read};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = 2^(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

fn build(aa: &[i32], n: usize) -> Vec<i32> {
    let mut st = vec![0; 2 * N_];
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[2 * i], st[2 * i + 1]);
    }
    st
}

fn query(st: &[i32], l: i32, a: i32) -> i32 {
    let mut l = l + N_ as i32;
    let mut r = N_ as i32 - 1;
    while l <= r {
        if (l & 1) == 1 {
            if st[l as usize] >= a {
                while l < N_ as i32 {
                    l = if st[2 * l as usize] >= a {
                        2 * l
                    } else {
                        2 * l + 1
                    };
                }
                return l - N_ as i32;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    N_ as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<i32>().unwrap();
    while t > 0 {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n = parts.next().unwrap().parse::<i32>().unwrap();
        let kl = parts.next().unwrap().parse::<i32>().unwrap();
        let kr = parts.next().unwrap().parse::<i32>().unwrap();
        let mut cc = parts.next().unwrap().chars().collect::<Vec<char>>();
        let mut zz = vec![0; n as usize];
        let mut ans = vec![0; (n + 1) as usize];

        for i in 1..n as usize {
            if i + zz[i - 1] < zz.len() {
                zz[i] = zz[i - 1];
            } else {
                let l = i;
                let mut r = max(zz.len(), i);
                while r < zz.len() && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let st = build(&zz, n as usize);
        for p in 1..=n {
            let mut k = 0;
            for i in 0..n as i32 {
                k += 1;
                i = query(&st, i, p);
            }
            ans[k as usize] = max(ans[k as usize], p);
        }

        for k in (1..n as usize).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
        t -= 1;
    }
}