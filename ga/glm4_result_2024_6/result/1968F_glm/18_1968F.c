use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200_000;

fn rand() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand() {
    let now = SystemTime::now();
    let duration = now.elapsed().unwrap();
    unsafe {
        Z = (duration.as_secs() as u32) ^ (duration.subsec_nanos() as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand() as usize % (r - l)];
        while j < k {
            let c = if super::aa[ii[j]] != super::aa[i_] {
                super::aa[ii[j]] - super::aa[i_]
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
        sort(ii, k, r);
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
    srand();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        super::n = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        super::aa.clear();
        super::aa.extend((0..=super::n).map(|_| 0));
        for i in 1..=super::n {
            super::aa[i] = parts.next().unwrap().parse().unwrap();
            super::aa[i] ^= super::aa[i - 1];
        }

        super::ii.clear();
        super::ii.extend((0..=super::n).map(|i| i));

        sort(&mut super::ii, 0, super::n + 1);

        for _ in 0..q {
            line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap() - 1;

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