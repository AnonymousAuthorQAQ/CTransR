use std::cmp::max;
use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18;

fn build(aa: &Vec<i32>, n: usize, st: &mut Vec<i32>, n_: usize) {
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
}

fn query(l: usize, a: i32, st: &Vec<i32>, n_: usize) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1 + n_;
    let mut result = n_;
    
    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut idx = l;
                while idx < n_ {
                    idx = if st[idx << 1] >= a { idx << 1 } else { idx << 1 | 1 };
                }
                return idx - n_;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: usize = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<&str> = iterator.next().unwrap().split_whitespace().collect();
        let n: usize = input[0].parse().unwrap();
        let kl: usize = input[1].parse().unwrap();
        let kr: usize = input[2].parse().unwrap();
        let cc: Vec<char> = input[3].chars().collect();
        
        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];
        
        let mut l = 0;
        let mut r = 0;
        
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = r.max(i);
                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }
        
        let mut st = vec![0; N_ * 2];
        build(&zz, n, &mut st, N_);
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = query(i + p, p as i32, &st, N_);
                if i < n {
                    k += 1;
                }
            }
            ans[k] = ans[k].max(p);
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