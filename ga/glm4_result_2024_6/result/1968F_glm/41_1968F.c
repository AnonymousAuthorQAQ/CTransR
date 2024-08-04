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
    let sec = duration.as_secs() as u32;
    let usec = duration.subsec_micros() as u32;
    unsafe {
        Z = sec ^ usec | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand() as usize % (r - l)];
        while j < k {
            let c = if ii[j] != i_ { ii[j] - i_ } else { ii[j] - i_ };
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
    let mut upper = N as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if a < ii[h as usize] || (a == ii[h as usize] && i < ii[h as usize]) {
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
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = iter.next().unwrap().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap() - 1;
            let r: usize = iter.next().unwrap().parse().unwrap();

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