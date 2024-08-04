use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH).unwrap();
        let z = (duration.as_secs() ^ duration.subsec_micros() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(aa: &[u32], ii: &mut Vec<usize>, l: usize, r: usize, rng: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (rng.rand() % (r - l) as u32) as usize];

        let mut j = l;
        let mut k = r;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
            } else {
                ii[j] as i32 - i_ as i32
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
        sort(aa, ii, l, k, rng);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = ii.len() as isize;

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
    let mut rng = Random::new();
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        for i in 1..=n {
            aa[i] = parts.next().unwrap().parse::<u32>().unwrap();
            aa[i] ^= aa[i - 1];
        }

        sort(&aa, &mut ii, 0, ii.len() - 1, &mut rng);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.split_whitespace();
            let l: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse::<usize>().unwrap();

            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) as usize + 1] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}