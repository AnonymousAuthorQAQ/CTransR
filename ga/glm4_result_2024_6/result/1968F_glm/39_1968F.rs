use std::io::{self, Read};

const N: usize = 200000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z
    }
}

fn srand_() {
    let mut tv = std::time::SystemTime::now();
    unsafe {
        Z = tv.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32 ^ tv.subsec_nanos() as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        let mut tmp;
        while j < k {
            let c = if crate::aa::aa[ii[j]] != crate::aa::aa[i_] {
                crate::aa::aa[ii[j]] - crate::aa::aa[i_]
            } else {
                ii[j] - i_
            };
            if c == 0 {
                j += 1;
            } else if c < 0 {
                tmp = ii[i];
                ii[i] = ii[j];
                ii[j] = tmp;
                i += 1;
                j += 1;
            } else {
                k -= 1;
                tmp = ii[j];
                ii[j] = ii[k];
                ii[k] = tmp;
            }
        }
        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: u32, i: usize) -> usize {
    let mut lower = -1;
    let mut upper = N + 1;
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if crate::aa::aa[crate::aa::ii[h]] < a || (crate::aa::aa[crate::aa::ii[h]] == a && crate::aa::ii[h] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

mod aa {
    use std::cell::RefCell;
    pub static mut aa: Vec<u32> = Vec::with_capacity(N + 1);
    pub static mut ii: Vec<usize> = Vec::with_capacity(N + 1);
}

fn main() {
    crate::srand_();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    for _ in 0..t {
        let mut lines = input.lines();
        let nq: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let n: usize = nq[0].parse().unwrap();
        let q: usize = nq[1].parse().unwrap();
        crate::aa::aa.clear();
        crate::aa::aa.resize(n + 1, 0);
        crate::aa::ii.clear();
        crate::aa::ii.resize(n + 1, 0);
        for i in 1..=n {
            crate::aa::aa[i] = lines.next().unwrap().parse().unwrap();
            crate::aa::aa[i] ^= crate::aa::aa[i - 1];
            crate::aa::ii[i] = i;
        }
        crate::sort(&mut crate::aa::ii, 0, n + 1);
        for _ in 0..q {
            let lr: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
            let l: usize = lr[0].parse().unwrap() - 1;
            let r: usize = lr[1].parse().unwrap();
            println!("{}", if crate::aa::aa[l] == crate::aa::aa[r] || crate::aa::ii[crate::search(crate::aa::aa[r], l) + 1] < crate::aa::ii[crate::search(crate::aa::aa[l], r)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}