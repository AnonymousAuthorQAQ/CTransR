use std::io;
use std::str;

const N: usize = 200_000;
const N_: usize = (1 << 18); // N_ = pow2(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

fn build(aa: &[i32], n: usize) -> Vec<i32> {
    let mut st = vec![0; N_ * 2];
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

fn query(st: &[i32], l: usize, a: i32) -> usize {
    let mut l = l + N_;
    let mut r = N_ - 1;
    while l <= r {
        if (l & 1) == 1 {
            if st[l] >= a {
                while l < N_ {
                    l = if st[2 * l] >= a { 2 * l } else { 2 * l + 1 };
                }
                return l - N_;
            }
            l += 1;
        } else {
            l >>= 1;
            r >>= 1;
        }
    }
    N_
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t = str::parse::<i32>(&input).expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.split_whitespace();
        let n: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let kl: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let kr: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let cc = tokens.next().unwrap();
        
        let mut zz = vec![0; n as usize];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n as usize {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = r.max(i);
                while r < n as usize && &cc[r as usize..] == &cc[l as usize..r as usize] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }
        
        let st = build(&zz, n as usize);
        let mut ans = vec![0; n as usize + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut l = 0;
            while l < n as usize {
                l = query(&st, l, p);
                k += 1;
            }
            ans[k] = ans[k].max(p);
        }
        
        for k in (1..n).rev() {
            ans[k] = ans[k].max(ans[k + 1]);
        }
        
        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
    }
}