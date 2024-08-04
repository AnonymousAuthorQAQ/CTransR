use std::io::{self, BufRead};
use rand::Rng;

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

struct Hashing {
    hash: Vec<usize>,
    base_pow: Vec<usize>,
}

impl Hashing {
    fn new(n: usize, base: usize, s: &[u8]) -> Self {
        let mut hash = vec![0; n + 1];
        let mut base_pow = vec![0; n + 1];
        
        base_pow[0] = 1;
        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
            hash[i] = (hash[i - 1] * base + s[i - 1] as usize) % MOD;
        }
        
        Hashing { hash, base_pow }
    }

    fn check(&self, k: usize, l: usize) -> usize {
        let mut count = 0;
        let n = self.hash.len() - 1;

        for i in 1..=n-k+1 {
            let hash_val = (self.hash[i + k - 1] + MOD - self.hash[i - 1]) % MOD;
            if hash_val == (self.hash[k] * self.base_pow[i - 1]) % MOD {
                count += 1;
                // Skip to i + k
                i += k - 1;
            }
        }
        count
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    let mut rng = rand::thread_rng();
    
    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = params[0];
        let l = params[1];
        let r = params[2];
        let s = lines.next().unwrap().unwrap().as_bytes();

        let base = rng.gen_range(200..MOD); // Random base between 200 and MOD-1
        let hashing = Hashing::new(n, base, s);

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = hashing.check(mid, l)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        print!("{}", lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = hashing.check(mid, i)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!(); // Newline after each test case output
    }
}