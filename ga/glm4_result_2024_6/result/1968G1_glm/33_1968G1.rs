use rand::Rng;
use std::io::{self, Read};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    rand::random::<u64>() * rand::random::<u64>()
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    rand::random::<u64>() % (r - l) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    while t > 0 {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap();

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];
        let bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            base_pow[i] = bp * rand2(200, MOD - 1) % MOD;
            hash[i] = base_pow[i] * s.as_bytes()[i - 1] as u64 % MOD;
            if i > 1 {
                hash[i] = (hash[i] + hash[i - 1]) % MOD;
            }
            bp = base_pow[i];
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

        println!();
        t -= 1;
    }
}