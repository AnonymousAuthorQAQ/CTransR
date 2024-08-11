use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let half = power(a, b / 2);
    if b % 2 == 0 {
        (half * half) % MOD
    } else {
        (half * half % MOD * a) % MOD
    }
}

fn check(s: &[usize], n: usize, k: usize, hash: &[usize], base_pow: &[usize]) -> usize {
    let mut vo = 0;
    let mut i = 0;
    
    while i + k <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i] % MOD) {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input_lines = reader.lines();
    
    let t: usize = input_lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = input_lines.next().unwrap().unwrap();
        let mut splits = line.split_whitespace();
        let n: usize = splits.next().unwrap().parse().unwrap();
        let l: usize = splits.next().unwrap().parse().unwrap();
        let r: usize = splits.next().unwrap().parse().unwrap();
        let s: String = input_lines.next().unwrap().unwrap();
        
        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];
        
        // Random base in range [200, MOD - 1]
        let mut rng = rand::thread_rng();
        let base = rng.gen_range(200..MOD);
        
        base_pow[0] = 1;
        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
            hash[i] = (hash[i - 1] + (s.chars().nth(i - 1).unwrap() as usize)) % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;

        // Binary search for the maximum length of substring
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = check(&s.as_bytes(), n, mid, &hash, &base_pow);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        let mut yy = check(&s.as_bytes(), n, lower, &hash, &base_pow);
        
        print!("{}", lower);
        
        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(&s.as_bytes(), n, mid, &hash, &base_pow);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!(" {}", lower);
        }
        println!();
    }
}