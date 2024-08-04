use std::io::{self, BufRead};
use rand::Rng;

const N: usize = 200005;
const MOD: i64 = 1000000007;

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

fn check(s: &[i64], hash: &[i64], base_pow: &[i64], n: usize, k: usize) -> i64 {
    let mut vo = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1]) % MOD {
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
    let reader = stdin.lock();

    let mut rng = rand::thread_rng();
    let base: i64 = rng.gen_range(200..MOD - 1);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<i64> = input.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = parts[0] as usize;
        let l = parts[1] as usize;
        let r = parts[2] as usize;
        let s: Vec<i64> = input.trim().chars().map(|c| (c as i64 - 'a' as i64 + 1) % MOD).collect();
        
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let mut bp = 1;

        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow[i] = bp;
            hash[i] = (bp * s[i - 1]) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(&s, &hash, &base_pow, n, mid);
            if yy >= l as i64 {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{}", lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;

                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(&s, &hash, &base_pow, n, mid);
                    if yy >= i as i64 {
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