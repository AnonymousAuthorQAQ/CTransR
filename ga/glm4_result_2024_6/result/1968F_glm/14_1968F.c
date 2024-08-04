use std::io::{self, BufRead};

const N: usize = 200000;

fn rand() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand() {
    let mut tv = std::time::SystemTime::now();
    let since_the_epoch = tv.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
    let tv_sec = since_the_epoch.as_secs();
    let tv_usec = since_the_epoch.subsec_micros();
    unsafe {
        Z = (tv_sec as u32) ^ (tv_usec as u32) | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize]) {
    if aa.len() <= 1 {
        return;
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = aa.len();
    let i_ = ii[rand() as usize % (k - i)];
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
    sort(&mut aa[..i], &mut ii[..i]);
    sort(&mut aa[i..], &mut ii[i..]);
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = ii.len() + 1;

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
            line = lines.next().unwrap().unwrap();
            parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap();

            let l_val = aa[l];
            let r_val = aa[r];
            let l_idx = search(l_val, l, &aa, &ii);
            let r_idx = search(r_val, r, &aa, &ii);

            println!(
                "{}",
                if l_val != r_val || ii[l_idx + 1] < ii[r_idx] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}