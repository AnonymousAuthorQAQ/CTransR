use std::io::{self, BufRead};
use rand::Rng;

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let half = power(a, b / 2);
    let result = half * half % MOD;
    if b % 2 == 1 {
        result * a % MOD
    } else {
        result
    }
}

fn check(n: usize, k: usize, hash: &[usize], base_pow: &[usize], s: &[u8]) -> usize {
    let mut count = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        let hash_i_k = (hash[i + k - 1] + MOD - hash[i - 1]) % MOD;
        let hash_k = (hash[k] * base_pow[i - 1]) % MOD;

        if hash_i_k == hash_k {
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
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: Vec<u8> = ["0".to_string(), parts.next().unwrap().to_string()].concat().into_bytes();

        let mut rng = rand::thread_rng();
        let base: usize = rng.gen_range(200..MOD as usize);

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];

        base_pow[0] = 1;
        let mut bp = 1;
        
        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow[i] = bp;
            hash[i] = (bp * (s[i] as usize) % MOD + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(n, mid, &hash, &base_pow, &s);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        let mut output = vec![lower.to_string()];

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(n, mid, &hash, &base_pow, &s);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            output.push(lower.to_string());
        }
        
        println!("{}", output.join(" "));
    }
}