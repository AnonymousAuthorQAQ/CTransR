use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200_000;

fn rand() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = (Z.wrapping_mul(3)).wrapping_shr(1);
        Z
    }
}

fn srand() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let tv_sec = since_epoch.as_secs() as u32;
    let tv_usec = since_epoch.subsec_micros() as u32;
    unsafe {
        Z = tv_sec ^ tv_usec | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize]) {
    let mut l = 0;
    let mut r = aa.len();
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand() as usize % (r - l)];
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
        sort(aa, &mut ii[l..i]);
        l = k;
    }
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = aa.len() + 1;
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h]] < a || (aa[ii[h]] == a && ii[h] < i) {
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
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();
        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();
        for i in 1..=n {
            aa[i] = parts.next().unwrap().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }
        sort(&mut aa, &mut ii);
        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap();
            let l = search(aa[l], l, &aa, &ii);
            let r = search(aa[r], r, &aa, &ii);
            println!("{}", if aa[l] == aa[r] || ii[l + 1] < ii[r] { "YES" } else { "NO" });
        }
    }
}