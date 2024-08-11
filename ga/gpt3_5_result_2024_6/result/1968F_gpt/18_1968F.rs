use std::io::{self, BufRead};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z as i32
    }
}

fn srand_() {
    unsafe {
        let mut tv = libc::timeval { tv_sec: 0, tv_usec: 0 };
        libc::gettimeofday(&mut tv, std::ptr::null_mut());
        Z = tv.tv_sec ^ tv.tv_usec as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + rand_() as usize % (r - l)]];

        while j < k {
            let c = if aa[ii[j]] != i_ { aa[ii[j]] - i_ } else { ii[j] - i_ };

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
        sort(ii, l, i, aa);
        l = k;
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> i32 {
    let mut lower = -1;
    let mut upper = ii.len() as i32 + 1;

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
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    let t: usize = iterator.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line: String = iterator.next().unwrap().trim().to_string();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa: Vec<u32> = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        let line: String = iterator.next().unwrap().trim().to_string();
        let mut iter = line.split_whitespace();
        for i in 1..=n {
            aa[i] = iter.next().unwrap().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            let line: String = iterator.next().unwrap().trim().to_string();
            let mut iter = line.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();
            let i = l - 1;

            let result = if aa[l] == aa[r] || ii[(search(aa[r], i, &ii, &aa) + 1) as usize] < ii[search(aa[l], r, &ii, &aa) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}