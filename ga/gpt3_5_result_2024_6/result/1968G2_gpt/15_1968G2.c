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

fn build(aa: &mut [usize], n: usize) -> Vec<usize> {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;
    
    while n_ < n {
        n_ <<= 1;
    }
    
    st[n_..n_ + n].copy_from_slice(aa);
    
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
    
    st
}

fn query(st: &[usize], n_: usize, l: usize, a: usize) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1;
    
    while l <= r {
        if (l & 1) == 1 {
            if st[l] >= a {
                let mut temp = l;
                while temp < n_ {
                    temp = if st[temp << 1] >= a {
                        temp << 1
                    } else {
                        temp << 1 | 1
                    };
                }
                return temp - n_;
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
    
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc = iter.next().unwrap();
        
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
        
        let st = build(&mut zz, n);
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