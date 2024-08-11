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
    let timeval = nix::time::TimeVal::now();
    unsafe {
        Z = timeval.sec() as u32 ^ timeval.usec() as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    if l < r {
        let i_ = ii[l + (rand_() as usize) % (r - l)];
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] { aa[ii[j]] - aa[i_] } else { ii[j] - i_ };

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
        sort(ii, k, r, aa);
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> isize {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as isize;

    while upper - lower > 1 {
        let h = ((lower + upper) / 2) as usize;

        if aa[ii[h]] < a || (aa[ii[h]] == a && ii[h] < i) {
            lower = h as isize;
        } else {
            upper = h as isize;
        }
    }

    lower
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();
    let t: i32 = buffer.trim().parse().unwrap();

    srand_();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let aa: Vec<u32> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut aa_prefixsum = vec![0; n + 1];
        for i in 1..=n {
            aa_prefixsum[i] = aa_prefixsum[i - 1] ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        
        sort(&mut ii, 0, n + 1, &aa_prefixsum);

        for _ in 0..q {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            let mut iter = buffer.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            let result = if aa_prefixsum[l] == aa_prefixsum[r] || ii[search(aa_prefixsum[r], l, &ii, &aa_prefixsum) as usize + 1] < ii[search(aa_prefixsum[l], r, &ii, &aa_prefixsum) as usize] { "YES" } else { "NO" };
            println!("{}", result);
        }
    }
}