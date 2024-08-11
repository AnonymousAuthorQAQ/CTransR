use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> u32 {
    unsafe {
        Z *= 3;
        Z >> 1
    }
}

fn srand_() {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let tv_sec = since_epoch.as_secs() as u32;
    let tv_usec = since_epoch.subsec_micros() as u32;
    unsafe {
        Z = tv_sec ^ tv_usec | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (rand_() as usize) % (r - l)];

        let mut j = j;
        let mut k = k;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as isize - aa[i_] as isize
            } else {
                ii[j] as isize - i_ as isize
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                j += 1;
                l += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, l, j, aa);
        l = k;
    }
}

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = (aa.len() - 1) as isize;

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
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();
        
        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();
        
        for i in 1..=n {
            let value: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ value;
        }
        
        sort(&mut ii, 0, n + 1, &aa);
        
        for _ in 0..q {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let mut l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();
            l -= 1; // Adjusting index to be zero-based
            
            if aa[l] == aa[r] || ii[(search(aa[r], l, &aa, &ii) + 1) as usize] < ii[search(aa[l], r, &aa, &ii) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}