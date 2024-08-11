use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;

    while n_ < n {
        n_ <<= 1;
    }

    st[n_..n_+n].clone_from_slice(&aa);

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }

    st
}

fn query(st: &Vec<usize>, n_: usize, l: usize, a: usize) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1 + n_;

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
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        input.read_line(&mut buffer).unwrap();
        let parts: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let kl = parts[1];
        let kr = parts[2];
        let cc = parts[3].trim().to_string();

        let mut zz = vec![0; n];
        let cc_chars: Vec<char> = cc.chars().collect();

        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);

                while r < n && cc_chars[r] == cc_chars[r - l] {
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