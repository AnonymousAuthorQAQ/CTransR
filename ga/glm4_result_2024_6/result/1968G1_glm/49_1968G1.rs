use rand::{Rng, SeedableRng, rngs::StdRng};
use std::time::SystemTime;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    let mut rng = StdRng::from_entropy();
    rng.gen()
}

fn rand2(l: i64, r: i64) -> i64 {
    if l == r { return l; }
    rand1() % (r - l) + l
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 { return 1; }
    let r = power(a, b / 2);
    if b & 1 == 1 { return r * r % MOD * a % MOD; }
    r * r % MOD
}

fn check(k: i64, hash: &[i64], base_pow: &[i64]) -> i64 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= hash.len() as i64 {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k as usize] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let mut rng = StdRng::from_entropy();
    let base = rng.gen_range(200..MOD);
    let mut base_pow = vec![0; N + 1];
    base_pow[0] = 1;

    let t = rand1();
    for _ in 0..t {
        let n = rand2(1, N as i64);
        let l = rand2(1, n);
        let r = rand2(l, n);
        let s = (0..n).map(|_| rng.gen_range(b'a'..b'z')).collect::<String>();

        let mut hash = vec![0; N];
        let mut bp = 1;
        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow[i] = bp;
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