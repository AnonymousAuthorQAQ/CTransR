use std::io::{self, Read};
use rand::Rng;
use rand::distributions::Range;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    rand::random::<i64>() * 1
}

fn rand2(l: i64, r: i64) -> i64 {
    if l == r {
        return l;
    }
    rand::random::<i64>() % (r - l) + l
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: i64, hash: &Vec<i64>, base_pow: &Vec<i64>) -> i64 {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: i64 = lines.next().unwrap().parse().unwrap();

    while t > 0 {
        let nlr = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap());
        let (n, l, r) = (nlr.next().unwrap(), nlr.next().unwrap(), nlr.next().unwrap());
        let s = lines.next().unwrap();

        let mut base_pow = vec![1; n as usize + 1];
        let mut hash = vec![0; n as usize + 1];
        let mut bp = 1;
        for i in 1..=n as usize {
            bp = bp * rand2(200, MOD - 1) % MOD;
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
        t -= 1;
    }
}