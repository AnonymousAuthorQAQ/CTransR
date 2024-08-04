use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = 2^(ceil(log2(N)))

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
    st.iter_mut().for_each(|x| *x = 0);
    
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
    st
}

fn query(l: usize, a: i32, st: &Vec<i32>, n_: usize) -> usize {
    let mut l = l;
    let mut r = n_ - 1;

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut x = l;
                while x < n_ {
                    x = if st[x << 1] >= a { x << 1 } else { x << 1 | 1 };
                }
                return x - n_;
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
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: i32 = iter.next().unwrap().parse().unwrap();
        let kr: i32 = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().parse().unwrap();
        
        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);

                while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
                    r += 1;
                }
                zz[i] = (r - l) as i32;
            }
        }

        let st = build(&zz, n);
        
        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            
            while i < n {
                i = query(i + p, p as i32, &st, n_);
                if i < n {
                    k += 1;
                }
            }
            ans[k as usize] = cmp::max(ans[k as usize], p);
        }
        
        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
    }
}