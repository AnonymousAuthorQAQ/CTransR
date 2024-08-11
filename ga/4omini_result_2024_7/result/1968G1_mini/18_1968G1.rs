use rand::Rng;
use std::io::{self, Read};

const N: usize = 200005;
const MOD: i32 = 1000000007;

fn power(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r as i64 * r as i64 % MOD as i64 * a as i64 % MOD as i64 as i32;
    }
    return r as i64 * r as i64 % MOD as i64 as i32;
}

fn check(k: usize, n: usize, s: &[i32], hash: &[i32], base_pow: &[i32]) -> i32 {
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
    let mut rng = rand::thread_rng();
    let base: i32 = rng.gen_range(200..MOD);
    
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = params[0];
        let l = params[1];
        let r = params[2];
        let s: Vec<i32> = lines.next().unwrap().chars().map(|c| c as i32).collect();
        
        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];

        base_pow[0] = 1;
        let mut bp = 1;
        for i in 1..=n {
            bp = (bp as i64 * base as i64 % MOD as i64) as i32;
            base_pow[i] = bp;
            hash[i] = (bp as i64 * s[i - 1] as i64 % MOD as i64) as i32;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &s, &hash, &base_pow)) >= l {
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
                    if (yy = check(mid, n, &s, &hash, &base_pow)) >= i {
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