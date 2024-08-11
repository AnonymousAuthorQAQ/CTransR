use std::io::{self, BufRead};
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut rng = rand::thread_rng();

    while t > 0 {
        let mut input = lines.next().unwrap().unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: i32 = parts.next().unwrap().parse().unwrap();
        let r: i32 = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap();

        let mut hash = vec![0; n + 1];
        let mut base_pow = vec![0; n + 1];
        let base = rng.gen_range(200..MOD as usize);
        let mut bp = 1;

        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp.wrapping_mul(base as i64) % MOD;
            base_pow[i] = bp;
            hash[i] = (bp.wrapping_mul(s.as_bytes()[i - 1] as i64)) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, &hash, &base_pow, n)) >= l {
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
                    if (yy = check(mid, &hash, &base_pow, n)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            println!("{}", lower);
        }

        println!();
        t -= 1;
    }
}

fn check(k: usize, hash: &Vec<i64>, base_pow: &Vec<i64>, n: usize) -> i32 {
    let mut vo = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }

    vo
}