use rand::prelude::*;
use std::io::{self, BufRead};
use std::str::FromStr;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    rand::random::<i64>() * rand::random::<i64>() % MOD
}

fn rand2(l: i64, r: i64) -> i64 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, hash: &[i64], base_pow: &[i64]) -> usize {
    let mut vo = 0;
    let n = hash.len();
    for i in 1..=n {
        let end = i + k - 1;
        if end <= n && (hash[end] + MOD - hash[i - 1]) % MOD == (hash[k - 1] * base_pow[i - 1]) % MOD {
            vo += 1;
            i += k;
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let base: i64 = rng.gen_range(200..MOD);
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let l: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let r: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let s = lines.next().unwrap().unwrap();
        
        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];
        let mut bp = 1;
        
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow[i] = bp;
            hash[i] = bp * s.chars().nth(i - 1).unwrap() as i64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i > check(lower, &hash, &base_pow) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow) >= i {
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