use rand::Rng;
use std::time::SystemTime;

fn rand1() -> u32 {
    rand::random()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

const N: usize = 200005;
const MOD: u64 = 1000000007;

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
    let t = 1; // This should be replaced with the actual number of test cases.

    for _ in 0..t {
        let n = 5; // Replace with the actual length of the string.
        let l = 2; // Replace with the actual minimum number of times a substring should appear.
        let r = 4; // Replace with the actual range of queries for the length of the substring.
        let s = "abcde"; // Replace with the actual string.

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];

        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * rand2(200, MOD as u32) as u64 % MOD;
            base_pow[i] = bp;
            hash[i] = bp * s.chars().nth(i - 1).unwrap() as u64 % MOD;
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