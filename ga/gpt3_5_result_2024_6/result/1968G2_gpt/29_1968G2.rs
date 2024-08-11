use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = 2^ceil(log2(N))

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn build(st: &mut [i32], aa: &[i32]) {
    let mut n_ = 1;
    while n_ < aa.len() {
        n_ <<= 1;
    }
    st.iter_mut().for_each(|x| *x = 0);
    aa.iter().enumerate().for_each(|(i, &val)| st[n_ + i] = val);
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }
}

fn query(st: &[i32], n_: usize, l: usize, a: i32) -> usize {
    let mut r = n_ - 1;
    let mut l_val = l;
    
    let mut l = l + n_;
    let mut r = r + n_;
    while l <= r {
        if l & 1 == 1 {
            if st[l] >= a {
                while l < n_ {
                    l = if st[l << 1] >= a { l << 1 } else { (l << 1) + 1 };
                }
                return l - n_;
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
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let kl: i32 = iter.next().unwrap().parse().unwrap();
        let kr: i32 = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().to_string();

        let mut zz = vec![0; n as usize];
        let cc_chars: Vec<char> = cc.chars().collect();

        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i as usize - l as usize] < r {
                zz[i as usize] = zz[i as usize - l as usize];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n as usize && cc_chars[r] == cc_chars[r - l] {
                    r += 1;
                }
                zz[i as usize] = (r - l) as i32;
            }
        }

        let mut st = vec![0; N_ * 2];
        build(&mut st, &zz);

        let mut ans = vec![0; (n as usize + 1)];
        for p in 1..=n as usize {
            let mut k = 0;
            let mut i = 0;
            while i < n as usize {
                i = query(&st, N_, i + p, p as i32);
                if i < n as usize {
                    k += 1;
                }
            }
            ans[k] = cmp::max(ans[k], p);
        }

        for k in (1..=n as usize - 1).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }

        for k in kl as usize..=kr as usize {
            print!("{} ", ans[k]);
        }
        println!();
    }
}