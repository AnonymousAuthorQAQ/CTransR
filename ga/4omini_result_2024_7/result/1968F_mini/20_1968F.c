use std::io;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        // Seed with current time
        let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        let sec = start.as_secs() as u64;
        let usec = start.subsec_micros() as u64;
        let seed = (sec ^ usec as u64) as u32 | 1;
        Random { z: seed }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.overflowing_mul(3).0;
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, random: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (random.rand() % (r - l) as u32) as usize];

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]].wrapping_sub(aa[i_])
            } else {
                ii[j].wrapping_sub(ii[i])
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
        sort(ii, aa, i, l, random);
        l = k;
    }
}

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize], n: usize) -> usize {
    let mut lower = 0;
    let mut upper = n + 1;

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
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut reader = stdin.lock();

    // Read number of test cases
    reader.read_line(&mut buf).unwrap();
    let t: usize = buf.trim().parse().unwrap();

    let mut random = Random::new();

    for _ in 0..t {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let mut parts = buf.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa = vec![0u32; n + 1];
        for i in 1..=n {
            buf.clear();
            reader.read_line(&mut buf).unwrap();
            let val: u32 = buf.trim().parse().unwrap();
            aa[i] = val ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut random);

        for _ in 0..q {
            buf.clear();
            reader.read_line(&mut buf).unwrap();
            let mut parts = buf.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap();

            let result = if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii, n)] < ii[search(aa[l], r, &aa, &ii, n)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}