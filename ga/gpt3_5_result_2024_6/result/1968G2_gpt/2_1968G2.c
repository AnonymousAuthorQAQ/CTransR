use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18; /* N_ = pow2(ceil(log2(N))) */

fn build(aa: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut st: Vec<usize> = vec![0; N_ * 2];
    let mut n_ = 1;
    
    while n_ < n {
        n_ <<= 1;
    }
    
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    
    for i in (1..n_).rev() {
        st[i] = cmp::max(st[i << 1], st[i << 1 | 1]);
    }
    
    st
}

fn query(st: &Vec<usize>, n_: usize, l: usize, a: usize) -> usize {
    let mut r = n_ - 1;
    let mut l = l + n_;
    let mut r = r + n_;
    
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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let n = parts[0];
        let kl = parts[1];
        let kr = parts[2];
        let cc = parts[3].to_string();
        
        let mut zz = vec![0; n];
        
        let mut l = 0;
        let mut r = 0;
        
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n && cc.chars().nth(r).unwrap() == cc.chars().nth(r - l).unwrap() {
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
                i = query(&st, n, i + p, p);
                k += 1;
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