use rand::{Rng, thread_rng};
use std::io;
use std::str;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let mut rng = thread_rng();
    rng.gen()
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: u64, mut b: u64) -> u64 {
    let mut r = 1;
    while b > 0 {
        if b & 1 == 1 {
            r = r * a % MOD;
        }
        b /= 2;
        a = a * a % MOD;
    }
    r
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
    let t = read_input();
    for _ in 0..t {
        let (n, l, r, s) = read_test_case();
        let mut bp = 1;
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let BASE = rand2(200, MOD - 1);

        for i in 1..=n {
            bp = bp * BASE % MOD;
            base_pow[i] = bp;
            hash[i] = (bp * s.chars().nth(i - 1).unwrap() as u64) % MOD;
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

fn read_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    str::parse::<usize>(input.trim()).unwrap()
}

fn read_test_case() -> (usize, usize, usize, String) {
    let n = read_input();
    let l = read_input();
    let r = read_input();
    let s = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };
    (n, l, r, s)
}