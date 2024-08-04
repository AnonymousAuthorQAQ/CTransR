use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    rand::random::<i64>() * 1_i64.wrapping_shl(32) * rand::random::<i64>()
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
        if b % 2 == 1 {
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
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap();

        let mut bp = 1_i64;
        let mut base_pow = vec![1_i64; n + 1];
        let mut hash = vec![0_i64; n + 1];

        let base = rand2(200, MOD as i64);
        for i in 1..=n {
            base_pow[i] = bp * base % MOD;
            hash[i] = bp * s.as_bytes()[i - 1] as i64 % MOD;
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
    }
}