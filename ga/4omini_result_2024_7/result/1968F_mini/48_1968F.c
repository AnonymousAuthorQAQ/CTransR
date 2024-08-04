use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200_000;

struct RandomGenerator {
    z: u32,
}

impl RandomGenerator {
    fn new() -> Self {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let seed = (since_epoch.as_secs() ^ since_epoch.subsec_micros() as u64) as u32 | 1;
        
        RandomGenerator { z: seed }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &[u32], l: usize, r: usize, rng: &mut RandomGenerator) {
    let mut l = l;
    let mut r = r;
    
    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (rng.rand() % (r - l) as u32) as usize];
        
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
                j += 1;
                l += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, aa, i, l, rng);
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
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut rng = RandomGenerator::new();

    for _ in 0..t {
        let input: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = input[0];
        let q = input[1];

        let mut aa = vec![0u32; n + 1];
        for i in 1..=n {
            let value: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ value;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rng);

        for _ in 0..q {
            let input: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            let l = input[0] - 1;
            let r = input[1];

            let result = if aa[l] == aa[r] || ii[(search(aa[r], &ii, &aa, n, l) + 1) as usize] < 
                                                   ii[search(aa[l], &ii, &aa, n, r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}