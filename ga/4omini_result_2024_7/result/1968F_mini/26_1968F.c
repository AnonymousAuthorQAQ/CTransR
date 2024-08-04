use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, BufRead};

const N: usize = 200000;
static mut Z: u32 = 0;

fn rand_() -> u32 {
    unsafe { Z *= 3; Z >> 1 }
}

fn srand_() {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    unsafe { Z = (duration.as_secs() ^ duration.subsec_micros() as u64 as u32) | 1 }
}

fn sort(ii: &mut Vec<usize>, aa: &[u32], l: usize, r: usize) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let mut k = r;

        let i_ = aa[ii[l] as usize] ^ aa[ii[l] as usize % (r - l)];

        while j < k {
            let c = if aa[ii[j]] != i_ {
                aa[ii[j]] as i32 - i_ as i32
            } else {
                ii[j] as i32 - ii[i] as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                l += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, aa, i, l);
        l = k;
    }
}

fn search(a: u32, ii: &[usize], aa: &[u32], n: usize, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

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
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines();
    
    srand_();
    
    let t: usize = input.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = input.next().unwrap().unwrap();
            let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let mut val: u32 = input.next().unwrap().unwrap().parse().unwrap();
            aa[i] = val ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = input.next().unwrap().unwrap();
                let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                (nums.next().unwrap() - 1, nums.next().unwrap())
            };

            if aa[l] == aa[r] ||
                ii[search(aa[r], &ii, &aa, n, l) as usize + 1] < ii[search(aa[l], &ii, &aa, n, r as usize) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}