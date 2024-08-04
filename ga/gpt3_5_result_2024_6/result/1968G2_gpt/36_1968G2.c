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
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }
    
    st
}

fn query(st: &Vec<i32>, n_: usize, l: usize, a: i32) -> usize {
    let mut r = n_ - 1;
    let mut l = l + n_;
    let mut r = r + n_;
    
    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut temp_l = l;
                while temp_l < n_ {
                    temp_l = if st[temp_l << 1] >= a {
                        temp_l << 1
                    } else {
                        (temp_l << 1) + 1
                    };
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
        let mut iter = input.split_whitespace();
        
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: &str = iter.next().unwrap();
        
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
        
        let st = build(&mut zz, n);
        let mut ans = vec![0; n + 1];
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = query(&st, N_, i + p, p);
                if i < n_ {
                    k += 1;
                }
            }
            ans[k] = cmp::max(ans[k], p as i32);
        }
        
        for k in (1..=n - 1).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }
        
        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}