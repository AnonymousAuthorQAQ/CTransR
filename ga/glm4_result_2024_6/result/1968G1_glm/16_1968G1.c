use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    rand::random::<u64>() * rand::random::<u64>()
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        l
    } else {
        rand::random::<u64>() % (r - l + 1) + l
    }
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        1
    } else {
        let r = power(a, b / 2);
        if b % 2 == 1 {
            r * r % MOD * a % MOD
        } else {
            r * r % MOD
        }
    }
}

fn check(k: usize, hash: &[u64], base_pow: &[u64]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= hash.len() {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let BASE: u64 = rng.gen_range(200..MOD);
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap();
        
        let mut bp = 1;
        let mut base_pow = vec![0; n + 1];
        base_pow[0] = 1;
        let mut hash = vec![0; n + 1];
        
        for (i, c) in s.chars().enumerate() {
            bp = bp * BASE % MOD;
            base_pow[i + 1] = bp;
            hash[i + 1] = bp * c as u64 % MOD;
            hash[i + 1] = (hash[i + 1] + hash[i]) % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;
        
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, &hash, &base_pow)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);
        
        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(mid, &hash, &base_pow)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            println!("{}", lower);
        }
    }
}