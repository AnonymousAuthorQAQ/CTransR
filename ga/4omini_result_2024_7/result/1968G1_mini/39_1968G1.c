use rand::Rng;
use std::io::{self, BufRead};

// Constants
const N: usize = 200005;
const MOD: i32 = 1_000_000_007;

// Hash struct to hold data
struct Hash {
    hash: Vec<i32>,
    base_pow: Vec<i32>,
    base: i32,
}

impl Hash {
    fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let base = rng.gen_range(200..MOD);
        
        Hash {
            hash: vec![0; n + 1],
            base_pow: vec![0; n + 1],
            base,
        }
    }
    
    // Power calculation using modular exponentiation
    fn power(a: i32, b: i32) -> i32 {
        if b == 0 {
            return 1;
        }
        let half = Self::power(a, b / 2);
        let result = if b % 2 == 1 {
            half as i64 * half as i64 % MOD as i64 * a as i64 % MOD as i64
        } else {
            half as i64 * half as i64 % MOD as i64
        };
        result as i32
    }
    
    // Check function
    fn check(&self, s: &[i32], n: usize, k: usize) -> usize {
        let mut count = 0;
        let mut i = 1;

        while i + k - 1 <= n {
            if (self.hash[i + k - 1] + MOD - self.hash[i - 1]) % MOD ==
               self.hash[k] * self.base_pow[i - 1] % MOD {
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
    let mut lines = reader.lines();

    let cases: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..cases {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = first_line[0];
        let l = first_line[1];
        let r = first_line[2];

        let s: String = lines.next().unwrap().unwrap();
        let mut hash_data = Hash::new(n);

        let mut bp = 1;
        hash_data.base_pow[0] = 1;

        for i in 1..=n {
            bp = (bp as i64 * hash_data.base as i64 % MOD as i64) as i32;
            hash_data.base_pow[i] = bp;
            hash_data.hash[i] = (bp as i64 * (s.chars().nth(i - 1).unwrap() as i32) % MOD as i64) as i32;
            hash_data.hash[i] = (hash_data.hash[i] + hash_data.hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = hash_data.check(&hash_data.hash, n, mid)) >= l as usize {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;

                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = hash_data.check(&hash_data.hash, n, mid)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }
        println!(); // New line after processing each test case
    }
}