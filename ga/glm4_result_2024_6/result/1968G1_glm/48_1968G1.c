use std::io::{self, BufRead};
use rand::{Rng, SeedableRng, RngCore};
use rand_chacha::ChaCha20Rng;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let mut rng = ChaCha20Rng::from_entropy();
    let base: i64 = rng.gen_range(200..MOD);

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: String = parts.next().unwrap().to_string();

        let mut hash = vec![0; n + 1];
        let mut base_pow = vec![0; n + 1];
        base_pow[0] = 1;
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = hash[i - 1] * base % MOD + s.chars().nth(i - 1).unwrap() as i64;
            hash[i] %= MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow, l) {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i > check(lower, &hash, &base_pow, i) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow, i) {
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

fn check(k: usize, hash: &Vec<i64>, base_pow: &Vec<i64>, target: usize) -> bool {
    let mut count = 0;
    for i in 1..=hash.len() - k {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            count += 1;
            if count >= target {
                return true;
            }
        }
    }
    false
}