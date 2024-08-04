use std::io::{self, BufRead};
use rand::Rng;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    let r = (r * r) % MOD;
    if b % 2 == 1 {
        (r * a) % MOD
    } else {
        r
    }
}

fn check(s: &[i64], hash: &[i64], base_pow: &[i64], n: usize, k: usize) -> usize {
    let mut vo = 0;
    let mut i = 0;
    while i + k <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i] % MOD + MOD) % MOD {
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
    let mut rng = rand::thread_rng();

    let t: usize = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let mut s: Vec<i64> = parts.next().unwrap().chars().map(|c| c as i64).collect();
        s.insert(0, 0); // to make the s[index] start from 1

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];

        let base: i64 = rng.gen_range(200..MOD - 1);
        base_pow[0] = 1;

        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
            hash[i] = (hash[i - 1] + base_pow[i] * s[i]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = check(&s, &hash, &base_pow, n, mid);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        print!("{}", lower);

        for i in (l + 1)..=r {
            if i > lower {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let yy = check(&s, &hash, &base_pow, n, mid);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!();
    }
}