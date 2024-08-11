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
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + (rand_() as usize) % (r - l)]];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != i_ {
                aa[ii[j]] - i_
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
        sort(ii, l, i, aa);
        l = k;
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> i32 {
    let mut lower = -1;
    let mut upper = (aa.len() + 1) as i32;

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
    unsafe {
        srand_();
    }

    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let aa: Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut ii: Vec<usize> = (0..=n).collect();
        let mut output = String::new();

        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            input.clear();
            stdin.lock().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            let result = if aa[l - 1] == aa[r] || ii[search(aa[r], l - 1, &ii, &aa) as usize + 1] < ii[search(aa[l - 1], r, &ii, &aa) as usize] {
                "YES"
            } else {
                "NO"
            };
            output.push_str(result);
            output.push('\n');
        }

        print!("{}", output);
    }
}