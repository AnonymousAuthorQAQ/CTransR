use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200_000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = (Z.wrapping_mul(3)).rotate_right(1);
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let duration = now.elapsed().unwrap();
    unsafe {
        Z = (duration.as_secs() as u32) ^ (duration.subsec_nanos() as u32) | 1;
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
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] - aa[i_]
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

fn search(a: i32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = N as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i) {
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

        for i in 1..=n {
            let mut line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            aa[i] = parts.next().unwrap().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        for i in 0..=n {
            ii[i] = i;
        }
        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap();

            println!(
                "{}",
                if aa[l] == aa[r] || ii[search(aa[r], l) as usize + 1] < ii[search(aa[l], r)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}