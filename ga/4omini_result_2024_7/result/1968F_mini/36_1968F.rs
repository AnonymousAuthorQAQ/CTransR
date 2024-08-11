use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let z = (start.as_secs() as u32) ^ (start.subsec_millis() as u32) | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, rng: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (rng.rand() % (r - l) as u32) as usize];
        let mut tmp;

        let mut j = j;
        let mut k: usize = k;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
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
        sort(ii, aa, l, i, rng);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize, n: usize) -> i32 {
    let mut lower = -1;
    let mut upper = n as i32 + 1;

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
    let mut lines = reader.lines();
    
    let mut rng = Random::new();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = lines.next().unwrap().unwrap().parse::<u32>().unwrap() ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rng);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = lines.next().unwrap().unwrap();
                let mut iter = line.split_whitespace();
                (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
            };

            let l = l - 1;
            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l, n) as usize + 1] < ii[search(&aa, &ii, aa[l], r, n) as usize] {
                "YES\n"
            } else {
                "NO\n"
            };
            print!("{}", result);
        }
    }
}