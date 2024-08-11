use std::io::{self, BufRead};
use rand::Rng;

const MOD: usize = 1_000_000_007;
const N: usize = 200_005;

struct Hashing {
    n: usize,
    hash: Vec<usize>,
    base_pow: Vec<usize>,
}

impl Hashing {
    fn new(n: usize) -> Self {
        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];
        hash[0] = 0;

        let mut rng = rand::thread_rng();
        let base = rng.gen_range(200..MOD) as usize;
        
        base_pow[0] = 1;
        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
        }

        Hashing { n, hash, base_pow }
    }

    fn compute_hash(&mut self, s: &str) {
        for i in 1..=self.n {
            self.hash[i] = (self.hash[i - 1] + s.chars().nth(i - 1).unwrap() as usize) % MOD;
            self.hash[i] = (self.hash[i] * self.base_pow[i]) % MOD;
        }
    }

    fn check(&self, s: &str, k: usize) -> usize {
        let mut occurrences = 0;
        let substring_hash = (self.hash[k] * self.base_pow[0]) % MOD;

        let mut i = 1;
        while i + k - 1 <= self.n {
            let curr_hash = (self.hash[i + k - 1] + MOD - self.hash[i - 1]) % MOD;
            if curr_hash == substring_hash {
                occurrences += 1;
                i += k;
            } else {
                i += 1;
            }
        }

        occurrences
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = params[0];
        let l = params[1];
        let r = params[2];
        let s = lines.next().unwrap();

        let mut hashing = Hashing::new(n);
        hashing.compute_hash(s);

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = hashing.check(s, mid)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{} ", lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = hashing.check(s, mid)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{} ", lower);
        }

        println!();
    }
}