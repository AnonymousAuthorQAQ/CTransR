use std::io::{self, BufRead};
use rand::Rng;

const N: usize = 200005;
const MOD: usize = 1000000007;

fn power(a: usize, mut b: usize) -> usize {
    let mut result = 1;
    let mut base = a;
    while b > 0 {
        if b % 2 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        b /= 2;
    }
    result
}

fn check(hash: &[usize], base_pow: &[usize], n: usize, k: usize, l: usize) -> usize {
    let mut count = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1]) % MOD {
            count += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: String = lines.next().unwrap().unwrap();
        
        let mut rng = rand::thread_rng();
        let base: usize = rng.gen_range(200..MOD as usize);
        
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] + s.as_bytes()[i - 1] as usize * base_pow[i] % MOD) % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, n, mid, l)) >= l {
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
                    if (yy = check(&hash, &base_pow, n, mid, l)) >= i {
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