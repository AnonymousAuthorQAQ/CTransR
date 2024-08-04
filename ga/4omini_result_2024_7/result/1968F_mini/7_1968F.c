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
        let z = (start.as_secs() ^ start.subsec_nanos() as u64) as u32 | 1; // Ensure non-zero seed
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (rand.rand() % (r - l) as u32) as usize];

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
                l += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, aa, l, i, rand);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, n: usize, i: usize) -> isize {
    let mut lower = -1isize;
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
    let mut input = stdin.lock().lines();
    
    let mut rand = Random::new();
    
    let t: usize = input.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line: Vec<usize> = input.next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = line[0];
        let q = line[1];
        
        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        for i in 1..=n {
            let a: u32 = input.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ a;
        }
        
        sort(&mut ii, &aa, 0, n + 1, &mut rand);
        
        for _ in 0..q {
            let l_r: Vec<usize> = input.next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            
            let l = l_r[0] - 1;
            let r = l_r[1];

            let result = if aa[l] == aa[r] || ii[(search(&aa, &ii, aa[r], n, l) + 1) as usize] < ii[search(&aa, &ii, aa[l], n, r) as usize] {
                "YES"
            } else {
                "NO"
            };
            
            println!("{}", result);
        }
    }
}