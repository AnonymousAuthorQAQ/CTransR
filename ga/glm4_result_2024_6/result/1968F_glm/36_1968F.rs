use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200_000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let duration = now.elapsed().unwrap();
    let micros = duration.as_micros() as u32;
    unsafe {
        Z = (duration.as_secs() as u32).wrapping_add(micros).wrapping_add(1);
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
            let c = if super::aa[ii[j]] != super::aa[i_] {
                super::aa[ii[j]] - super::aa[i_]
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
                ii[j] = super::ii[k];
                super::ii[k] = tmp;
            }
        }
        sort(ii, l, i);
        l = k;
    }
}

fn search(a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = super::n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if super::aa[super::ii[h as usize]] < a || (super::aa[super::ii[h as usize]] == a && super::ii[h as usize] < i) {
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

    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let q = iter.next().unwrap().parse::<usize>().unwrap();
        super::n = n;

        for i in 1..=n {
            super::aa[i] = iter.next().unwrap().parse::<u32>().unwrap();
            super::aa[i] ^= super::aa[i - 1];
        }

        for i in 0..=n {
            super::ii[i] = i;
        }
        sort(&mut super::ii, 0, n + 1);

        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let l = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let r = iter.next().unwrap().parse::<usize>().unwrap();
            println!(
                "{}",
                if super::aa[l] == super::aa[r] || super::ii[search(super::aa[r], l) as usize + 1] < super::ii[search(super::aa[l], r)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}