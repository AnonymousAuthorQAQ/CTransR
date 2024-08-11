use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

fn rand() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3);
        Z >>= 1;
        Z
    }
}

fn srand() {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let secs = since_the_epoch.as_secs() as u32;
    let usecs = since_the_epoch.subsec_micros() as u32;
    unsafe {
        Z = secs ^ usecs | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize]) {
    let mut i = 0;
    while i < aa.len() {
        let mut j = i;
        let mut k = aa.len();
        let i_ = ii[i + rand() as usize % (aa.len() - i)];
        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] - aa[i_]
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
        sort(aa, &mut ii[i..]);
        i = k;
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
    let t = stdin.lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let mut iter = line.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = iter.next().unwrap().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<_>>();
        sort(&mut aa, &mut ii);

        for _ in 0..q {
            let mut line = String::new();
            stdin.lock().read_line(&mut line).unwrap();
            let mut iter = line.trim().split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();
            let result = if aa[l - 1] == aa[r - 1] {
                true
            } else {
                let l_idx = search(aa[l - 1], l - 1, &aa, &ii);
                let r_idx = search(aa[r - 1], r - 1, &aa, &ii);
                r_idx < l_idx
            };
            println!("{}", if result { "YES" } else { "NO" });
        }
    }
}