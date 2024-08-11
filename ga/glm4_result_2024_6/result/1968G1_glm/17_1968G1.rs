use std::io::{self, Read};
use rand::Rng;
use std::time::SystemTime;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    rand::random::<i64>() * rand::random::<i64>()
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

fn check(k: i64, hash: &[i64], base_pow: &[i64], n: i64) -> i64 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[(i + k - 1) as usize] + MOD - hash[(i - 1) as usize]) % MOD == hash[k as usize] * base_pow[(i - 1) as usize] % MOD {
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

    let mut rng = rand::thread_rng();
    let base: i64 = rng.gen_range(200..MOD);
    let mut base_pow = vec![1; N + 1];
    let mut hash = vec![0; N + 1];

    while t > 0 {
        let mut parts = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap());
        let n = parts.next().unwrap();
        let l = parts.next().unwrap();
        let r = parts.next().unwrap();
        let s = lines.next().unwrap();

        let mut bp = 1;
        for i in 1..=n {
            base_pow[i] = bp * base % MOD;
            hash[i] = bp * s.chars().nth(i - 1).unwrap() as i64 % MOD;
            if i > 1 {
                hash[i] = (hash[i] + hash[i - 1]) % MOD;
            }
            bp = base_pow[i];
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow, n) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i > check(lower, &hash, &base_pow, n) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow, n) >= i {
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