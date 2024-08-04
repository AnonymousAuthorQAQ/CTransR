use rand::Rng;
use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn random_base() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(200..MOD) // Generates a random base in the range [200, MOD-1]
}

fn power(a: usize, mut b: usize) -> usize {
    let mut r = 1;
    let mut a = a;

    while b > 0 {
        if b % 2 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        b /= 2;
    }
    r
}

fn check(k: usize, n: usize, hash: &[usize], base_pow: &[usize], s: &[usize]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    
    while i + k - 1 <= n {
        let hash_sub = (hash[i + k - 1] + MOD - hash[i - 1]) % MOD;
        if hash_sub == (hash[k] * base_pow[i - 1]) % MOD {
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
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut rng = rand::thread_rng();
    
    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let (n, l, r) = (params[0], params[1], params[2]);
        
        let s: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
        
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        
        let base = random_base();
        
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] * base + (s[i - 1] as usize)) % MOD; // s is 0-indexed
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, n, &hash, &base_pow, &s) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        print!("{}", lower);

        let mut yy = check(lower, n, &hash, &base_pow, &s);
        
        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;

                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, n, &hash, &base_pow, &s) >= i {
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