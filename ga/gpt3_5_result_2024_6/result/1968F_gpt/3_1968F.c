use std::io::{self, BufRead};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand() -> i32 {
    unsafe {
        Z = (Z.wrapping_mul(3)) >> 1;
        Z as i32
    }
}

fn srand() {
    unsafe {
        let mut tv = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        libc::gettimeofday(&mut tv, std::ptr::null_mut());
        Z = (tv.tv_sec ^ tv.tv_usec as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + (rand() as usize) % (r - l)]];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != i_ {
                aa[ii[j]] as i32 - i_ as i32
            } else {
                ii[j] as i32 - i_ as i32
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
        sort(ii, l, i, aa);
        l = k;
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> i32 {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as i32;

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
    srand();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line: String = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa: Vec<u32> = vec![0; n + 1];
        for i in 1..=n {
            let val: u32 = parts.next().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ val;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            let query: String = lines.next().unwrap();
            let mut parts = query.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();

            let result = if aa[l] == aa[r] || ii[(search(aa[r], l - 1, &ii, &aa) + 1) as usize] < ii[search(aa[l], r, &ii, &aa) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}