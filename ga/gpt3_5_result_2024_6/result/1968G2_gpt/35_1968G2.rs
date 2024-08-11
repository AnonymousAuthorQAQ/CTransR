use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18;  // N_ = 2^ceil(log2(N))

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
        st[i] = max(st[i << 1 | 0], st[i << 1 | 1]);
    }

    st
}

fn query(l: usize, a: i32, st: &Vec<i32>, n_: usize) -> usize {
    let mut r = n_ - 1;
    let mut l = l;
    
    let mut r = match (r as i32 + l as i32) as usize {
        x if x % 2 == 1 => x / 2,
        x => x,
    };

    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                let mut l = l as usize;
                while l < n_ {
                    l = if st[l << 1 | 0] >= a {
                        l << 1 | 0
                    } else {
                        l << 1 | 1
                    };
                }
                return l - n_;
            }
            l += 1;
        }
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
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        let cc: String = parts[3].trim().to_string();
        
        let mut zz = vec![0; n];
        let cc_chars: Vec<char> = cc.chars().collect();
        
        let (mut l, mut r) = (0, 0);
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n && cc_chars[r] == cc_chars[r - l] {
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
                i = query(i + p, p as i32, &st, n);
                if i != n {
                    k += 1;
                }
            }
            ans[k as usize] = cmp::max(ans[k as usize], p);
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