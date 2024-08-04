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
    let tv = unsafe { libc::gettimeofday() };
    unsafe {
        Z = (tv.tv_sec ^ tv.tv_usec | 1) as u32;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[i32]) {
    let mut l = l;
    let mut r = r;
    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + rand_() as usize % (r - l)] as usize];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != i_ { aa[ii[j]] - i_ } else { ii[j] as i32 - i_ };

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

fn search(a: i32, i: i32, ii: &[usize], aa: &[i32]) -> i32 {
    let mut lower = -1;
    let mut upper = (aa.len() + 1) as i32;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] as i32 < i) {
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
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let t: i32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let line: Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let n = line[0] as usize;
        let q = line[1] as usize;

        let mut aa = vec![0; n + 1];
        let mut ii = vec![0; n + 1];

        for i in 1..=n {
            aa[i] = lines.next().unwrap().parse().unwrap() ^ aa[i - 1];
        }

        for i in 0..=n {
            ii[i] = i;
        }

        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            let query: Vec<i32> = lines.next().unwrap().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            let l = query[0] as usize - 1;
            let r = query[1] as usize;

            let search_result = search(aa[r], l as i32, &ii, &aa);
            let output = if aa[l] == aa[r] || ii[(search_result + 1) as usize] < ii[search(aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", output);
        }
    }
}