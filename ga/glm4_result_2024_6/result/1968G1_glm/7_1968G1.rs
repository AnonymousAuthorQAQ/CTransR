use rand::{Rng, thread_rng};
use std::time::SystemTime;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    thread_rng().gen_range(1..=MOD)
}

fn rand2(l: i64, r: i64) -> i64 {
    if l == r {
        return l;
    }
    thread_rng().gen_range(l..=r)
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(k: usize) -> i64 {
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
    let t = rand2(200, MOD - 1);
    let mut base_pow = vec![1; N + 1];
    let mut hash = vec![0; N + 1];

    let mut lower = 0;
    let mut upper = N + 1;
    while upper - lower > 1 {
        let mid = lower + (upper - lower) / 2;
        if check(mid) >= l {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    println!("{}", lower);

    for i in l + 1..=r {
        if i > yy {
            upper = lower + 1;
            lower = 0;
            while upper - lower > 1 {
                let mid = lower + (upper - lower) / 2;
                if check(mid) >= i {
                    lower = mid;
                } else {
                    upper = mid;
                }
            }
        }
        println!("{}", lower);
    }
}