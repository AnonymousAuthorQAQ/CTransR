use std::io;
use std::time::SystemTime;

const N: usize = 200_000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let micros = since_the_epoch.subsec_micros() as u32;
    unsafe {
        Z = micros ^ Z;
        Z *= 3;
        Z >>= 1;
        Z |= 1;
    }
    Z
}

fn sort(aa: &mut [i32], ii: &mut [usize]) {
    let mut n = aa.len();
    while n > 1 {
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && aa[ii[j]] <= aa[ii[i]] {
                j += 1;
            }
            if j == n {
                break;
            }
            let mut k = j;
            while k < n && aa[ii[k]] > aa[ii[i]] {
                k += 1;
            }
            if k == n {
                break;
            }
            let mut l = i;
            let mut r = k;
            while l < r {
                let mut t = ii[l];
                ii[l] = ii[r];
                ii[r] = t;
                l += 1;
                r -= 1;
            }
            i = j;
            n = k;
        }
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();
        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();
        for i in 1..=n {
            line.clear();
            io::stdin().read_line(&mut line).unwrap();
            aa[i] = line.trim().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }
        sort(&mut aa, &mut ii);
        for _ in 0..q {
            line.clear();
            io::stdin().read_line(&mut line).unwrap();
            let mut parts = line.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();
            println!("{}", if aa[l - 1] == aa[r - 1] || ii[search(&aa, l - 1, r - 1) + 1] < ii[search(&aa, r - 1, l - 1)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}

fn search(aa: &[i32], l: usize, r: usize) -> usize {
    let mut lower = -1;
    let mut upper = aa.len();
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[h] < aa[l] || (aa[h] == aa[l] && h < l) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}