use std::io::{self, BufRead};

fn rand() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand() {
    let mut tv = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        libc::gettimeofday(&mut tv, std::ptr::null_mut());
        rand::S.rand_seed_u64((tv.tv_sec as u64) ^ (tv.tv_usec as u64) | 1);
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    while l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + rand() as usize % (r - l)];
        while j < k {
            let c = if crate::aa::aa[ii[j]] != crate::aa::aa[i_] {
                crate::aa::aa[ii[j]] - crate::aa::aa[i_]
            } else {
                ii[j] - i_
            };
            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                i += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, l, i);
        l = k;
    }
}

fn search(a: i32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = crate::aa::n as isize + 1;
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if crate::aa::aa[crate::aa::ii[h as usize]] < a || (crate::aa::aa[crate::aa::ii[h as usize]] == a && crate::aa::ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    srand();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    while t > 0 {
        let nq: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = nq[0];
        let q = nq[1];
        crate::aa::aa.clear();
        crate::aa::n = n;
        for _ in 0..n {
            let a: i32 = lines.next().unwrap().unwrap().parse().unwrap();
            crate::aa::aa.push(a);
        }
        for i in 0..=n {
            crate::aa::ii[i] = i;
        }
        sort(&mut crate::aa::ii, 0, n + 1);
        for _ in 0..q {
            let lr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            let l = lr[0] - 1;
            let r = lr[1];
            println!(
                "{}",
                if crate::aa::aa[l] == crate::aa::aa[r] || crate::aa::ii[search(crate::aa::aa[r], l) as usize + 1] < crate::aa::ii[search(crate::aa::aa[l], r)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
        t -= 1;
    }
}