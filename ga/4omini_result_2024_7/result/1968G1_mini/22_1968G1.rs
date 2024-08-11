use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

struct Hashing {
    base: usize,
    base_pow: Vec<usize>,
    hash: Vec<usize>,
}

impl Hashing {
    fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let base = rng.gen_range(200..MOD as usize);
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
        }
        
        Hashing { base, base_pow, hash }
    }
    
    fn compute_hash(&mut self, s: &[char]) {
        for i in 1..=s.len() {
            self.hash[i] = (self.hash[i - 1] * self.base + s[i - 1] as usize) % MOD;
        }
    }
    
    fn check(&self, k: usize, l: usize) -> usize {
        let mut count = 0;
        let target = (self.hash[k] * self.base_pow[l - 1]) % MOD;
        
        let mut i = 1;
        while i + k - 1 <= l {
            if (self.hash[i + k - 1] + MOD - self.hash[i - 1]) % MOD == target {
                count += 1;
                i += k;
            } else {
                i += 1;
            }
        }
        count
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut params = input.split_whitespace();
        let n: usize = params.next().unwrap().parse().unwrap();
        let l: usize = params.next().unwrap().parse().unwrap();
        let r: usize = params.next().unwrap().parse().unwrap();
        
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s: Vec<char> = input.trim().chars().collect();
        
        let mut hashing = Hashing::new(n);
        hashing.compute_hash(&s);
        
        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;
        
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = hashing.check(mid, n)) >= l {
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
                    if (yy = hashing.check(mid, n)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }
        
        println!();
    }
}