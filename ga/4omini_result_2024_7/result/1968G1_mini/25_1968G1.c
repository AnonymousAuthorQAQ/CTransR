use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i64 = 1_000_000_007;

fn power(a: i64, mut b: i64) -> i64 {
    let mut result = 1;
    let mut base = a;

    while b > 0 {
        if b % 2 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        b /= 2;
    }
    result
}

fn check(n: usize, k: usize, s: &[char], hash: &[i64], base_pow: &[i64]) -> i64 {
    let mut vo = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1]) % MOD {
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
    
    let mut rng = rand::thread_rng();
    let base: i64 = rng.gen_range(200..MOD);

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: i64 = parts.next().unwrap().parse().unwrap();
        let r: i64 = parts.next().unwrap().parse().unwrap();
        let s: Vec<char> = lines.next().unwrap().unwrap().chars().collect();

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];

        base_pow[0] = 1;
        let mut bp = 1;
        for i in 1..=n {
            bp = (bp * base) % MOD;
            base_pow[i] = bp;
            hash[i] = (bp * s[i - 1] as i64) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(n, mid, &s, &hash, &base_pow)) >= l {
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
                    if (yy = check(n, mid, &s, &hash, &base_pow)) >= i {
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