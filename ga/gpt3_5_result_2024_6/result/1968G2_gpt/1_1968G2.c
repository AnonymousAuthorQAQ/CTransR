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
        st[i] = cmp::max(st[i << 1], st[i << 1 | 1]);
    }

    st
}

fn query(l: usize, a: i32, st: &Vec<i32>, n_: usize) -> usize {
    let mut r = n_ - 1;
    let mut l = l;
    let mut r = r;

    loop {
        if l <= r {
            if l & 1 == 1 {
                if st[l] >= a {
                    let mut l = l;
                    while l < n_ {
                        l = if st[l << 1] >= a { l << 1 } else { l << 1 | 1 };
                    }
                    return l - n_;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        } else {
            break;
        }
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
        let params: Vec<usize> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let n = params[0];
        let kl = params[1];
        let kr = params[2];
        let cc = params[3].trim().to_string();
        
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

            ans[k] = cmp::max(ans[k], p as i32);
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