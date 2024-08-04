use rand::Rng;
use std::time::SystemTime;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u64 {
    rand::random::<u64>() as u64
}

fn rand2(l: u64, r: u64) -> u64 {
    l + rand1() % (r - l + 1)
}

fn power(a: i64, b: i64) -> i64 {
    let mut r = 1;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        if b & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        b >>= 1;
    }
    r
}

fn check(k: usize, hash: &[i64], base_pow: &[i64]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= N {
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
    let t = rand2(1, 100);
    let mut rng = rand::thread_rng();
    let mut base = rand2(200, MOD as u64 - 1) as i64;
    let mut base_pow = vec![1; N + 1];
    let mut hash = vec![0; N + 1];

    for i in 1..=N {
        base_pow[i] = base_pow[i - 1] * base % MOD;
        hash[i] = hash[i - 1] + base_pow[i - 1] * s[i] as i64 % MOD;
        hash[i] %= MOD;
    }

    let mut lower = 0;
    let mut upper = N + 1;
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