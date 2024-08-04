use std::io::{self, BufRead};

const N: usize = 200000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand_() {
    let mut tv = std::time::SystemTime::now();
    let since_the_epoch = tv.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
    let tv_sec = since_the_epoch.as_secs();
    let tv_usec = since_the_epoch.subsec_micros();
    unsafe {
        Z = (tv_sec as u32) ^ (tv_usec as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        let mut tmp;

        while j < k {
            let c = if crate::aa[i_] != crate::aa[ii[j]] {
                crate::aa[i_] - crate::aa[ii[j]]
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
        l = k;
    }
}

fn search(a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = crate::n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if crate::aa[crate::ii[h as usize]] < a || (crate::aa[crate::ii[h as usize]] == a && crate::ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    srand_();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();
        crate::n = n;

        crate::aa = vec![0; n + 1];
        crate::ii = (0..=n).collect::<Vec<_>>();

        for i in 1..=n {
            crate::aa[i] = parts.next().unwrap().parse().unwrap();
            crate::aa[i] ^= crate::aa[i - 1];
        }

        sort(&mut crate::ii, 0, n + 1);

        for _ in 0..q {
            line = lines.next().unwrap().unwrap();
            parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap();

            println!(
                "{}",
                if crate::aa[l] == crate::aa[r] || crate::ii[search(crate::aa[r], l) as usize + 1] < crate::ii[search(crate::aa[l], r)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}

mod crate {
    use std::cell::RefCell;

    pub static mut Z: u32 = 0;
    pub static mut n: usize = 0;
    pub static mut aa: Vec<u32> = Vec::new();
    pub static mut ii: Vec<usize> = Vec::new();
}