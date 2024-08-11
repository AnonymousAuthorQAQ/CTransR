use rand::{thread_rng, Rng};
use std::time::SystemTime;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    let mut rng = thread_rng();
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
    let t = rand2(1, 100); // Generate a random number of test cases

    for _ in 0..t {
        let n = rand2(1, N as i64);
        let l = rand2(1, n as i64);
        let r = rand2(l, n as i64);
        let mut s = String::with_capacity(n as usize);
        for _ in 0..n {
            s.push(rand2('a' as i64, 'z' as i64) as char);
        }

        let mut bp = 1;
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let base = rand2(200, MOD - 1);
        for i in 1..=n {
            base_pow[i] = bp * base % MOD;
            hash[i] = bp * s.chars().nth(i - 1).unwrap() as i64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
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
    }
}