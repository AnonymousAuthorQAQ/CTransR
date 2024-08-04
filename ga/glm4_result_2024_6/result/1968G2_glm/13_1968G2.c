const N: usize = 200_000;
const N_: usize = 1 << 18; // N_ = 2^ceil(log2(N))

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

fn build(aa: &[i32], n: usize) -> Vec<i32> {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;

    while n_ < n {
        n_ <<= 1;
    }

    for (i, &a) in aa.iter().enumerate() {
        st[n_ + i] = a;
    }

    for i in (1..n_).rev() {
        st[i] = max(st[2 * i], st[2 * i + 1]);
    }

    st
}

fn query(st: &[i32], l: i32, a: i32) -> i32 {
    let mut l = l + N_ as i32;
    let r = N_ as i32 - 1;

    while l <= r {
        if l % 2 == 1 {
            if st[l as usize] >= a {
                while l < N_ as i32 {
                    l = if st[2 * l as usize] >= a { 2 * l } else { 2 * l + 1 };
                }
                return l - N_ as i32;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }

    N_ as i32
}

fn main() {
    let t = input::<i32>();

    for _ in 0..t {
        let n = input::<i32>();
        let kl = input::<i32>();
        let kr = input::<i32>();
        let cc: String = input();
        let mut zz: Vec<i32> = vec![0; n as usize];

        for i in 1..n {
            let l = if i + zz[i as usize - 1] < i { zz[i as usize - 1] } else { i };
            let r = std::cmp::max(i, zz[i as usize - 1]);
            while r < n && cc.chars().nth(r as usize) == cc.chars().nth(r as usize - l) {
                r += 1;
            }
            zz[i as usize] = r - l;
        }

        let st = build(&zz, n as usize);
        let mut ans = vec![0; n as usize + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                let q = query(&st, i, p);
                k += 1;
                i = q;
            }
            ans[k as usize] = max(ans[k as usize], p);
        }

        for k in (1..n).rev() {
            ans[k as usize] = max(ans[k as usize], ans[k as usize + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
    }
}

fn input<T: std::str::FromStr>() -> T {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().expect("Error reading line");
    line.trim().parse().ok().expect("Error parsing input")
}

fn input<T: std::str::FromStr>() -> Vec<T> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().expect("Error reading line");
    line.split_whitespace().map(|s| s.parse().ok().expect("Error parsing input")).collect()
}