use rand::Rng;
use std::time::SystemTime;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    rand::random::<i64>() * 1u64.wrapping_shl(32) as i64
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

fn check(k: usize, hash: &[i64], base_pow: &[i64]) -> i64 {
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
    let mut rng = rand::thread_rng();
    let base = rng.gen_range(200..MOD);
    let mut base_pow = vec![0; N + 1];
    base_pow[0] = 1;
    for i in 1..=N {
        base_pow[i] = base_pow[i - 1] * base % MOD;
    }

    let t = rng.gen_range(1..=1000);
    for _ in 0..t {
        let n = rng.gen_range(1..=N);
        let l = rng.gen_range(1..=n);
        let r = rng.gen_range(l..=n);
        let s = (0..n).map(|_| rng.gen_range(0..26) as u8).collect::<Vec<_>>();
        let s = s.iter().map(|&b| b as char).collect::<String>();

        let mut hash = vec![0; N];
        let mut bp = 1;
        for i in 1..=n {
            bp = bp * base % MOD;
            hash[i] = bp * s.as_bytes()[i - 1] as i64 % MOD;
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
        println!();
    }
}