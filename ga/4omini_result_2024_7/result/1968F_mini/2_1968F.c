use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u64 = 0;

fn rand_() -> u64 {
    unsafe {
        Z = Z.wrapping_mul(3);
        Z >> 1
    }
}

fn srand_() {
    let start = SystemTime::now();
    if let Ok(duration) = start.duration_since(UNIX_EPOCH) {
        unsafe {
            Z = (duration.as_secs() ^ duration.subsec_micros() as u64) | 1;
        }
    }
}

fn sort(ii: &mut [usize], aa: &[u64], l: usize, r: usize) {
    let mut left = l;
    let mut right = l;
    let mut k = r;

    while left < k {
        let i_ = aa[ii[l] as usize] ^ rand_() % (r - l) as u64;
        let mut i = left;

        while right < k {
            let c = if aa[ii[right]] != i_ {
                aa[ii[right]] - i_
            } else {
                ii[right] as isize - ii[i] as isize
            };

            if c == 0 {
                right += 1;
            } else if c < 0 {
                ii.swap(i, right);
                i += 1;
                right += 1;
            } else {
                k -= 1;
                ii.swap(right, k);
            }
        }
        sort(ii, aa, left, i);
        left = k;
    }
}

fn search(aa: &Vec<u64>, ii: &Vec<usize>, a: u64, n: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < 0) {
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
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let (n, q) = {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            (iter.next().unwrap().parse::<usize>().unwrap(), iter.next().unwrap().parse::<usize>().unwrap())
        };

        let mut aa = vec![0u64; n + 1];
        for i in 1..=n {
            aa[i] = lines.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
            aa[i] ^= aa[i - 1];
        }
        
        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1);
        
        for _ in 0..q {
            let (l, r) = {
                let line = lines.next().unwrap().unwrap();
                let mut iter = line.split_whitespace();
                (iter.next().unwrap().parse::<usize>().unwrap() - 1, iter.next().unwrap().parse::<usize>().unwrap())
            };

            if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], n)] < ii[search(&aa, &ii, aa[l], n)] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}