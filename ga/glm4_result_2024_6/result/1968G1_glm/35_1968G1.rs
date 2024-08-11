use std::io::{self, BufRead};
use rand::{thread_rng, Rng};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    let mut rng = thread_rng();
    rng.gen_range(0..=MOD)
}

fn rand2(l: i64, r: i64) -> i64 {
    rand1() % (r - l + 1) + l
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        r * r % MOD * a % MOD
    } else {
        r * r % MOD
    }
}

fn check(k: usize, hash: &[i64], base_pow: &[i64], n: usize) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    while t > 0 {
        let mut line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap();

        let bp = 1;
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let base = rand2(200, MOD as i64);
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

        t -= 1;
    }
}