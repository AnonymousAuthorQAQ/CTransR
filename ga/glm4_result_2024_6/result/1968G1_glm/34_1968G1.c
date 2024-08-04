use std::io::{self, Read};
use rand::{Rng, SeedableRng, RngCore};
use rand_chacha::ChaCha8Rng;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    rand::random::<i64>() * rand::random::<i64>()
}

fn rand2(l: i64, r: i64) -> i64 {
    if l == r {
        l
    } else {
        rand1() % (r - l) + l
    }
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        1
    } else {
        let r = power(a, b / 2);
        if b & 1 == 1 {
            r * r % MOD * a % MOD
        } else {
            r * r % MOD
        }
    }
}

fn check(k: usize, hash: &[i64], base_pow: &[i64]) -> usize {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: i32 = lines.next().unwrap().parse().unwrap();

    let mut rng = ChaCha8Rng::from_entropy();

    while t > 0 {
        let mut nums = lines.next().unwrap().split_whitespace();
        let n: usize = nums.next().unwrap().parse().unwrap();
        let l: i32 = nums.next().unwrap().parse().unwrap();
        let r: i32 = nums.next().unwrap().parse().unwrap();
        let s = nums.next().unwrap();

        let mut bp = 1;
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        let base: i64 = rng.gen_range(200..MOD);
        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow[i] = bp;
            hash[i] = bp * (s.as_bytes()[i - 1] as i64) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow) >= l as usize {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i as usize > check(lower, &hash, &base_pow) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow) >= i as usize {
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