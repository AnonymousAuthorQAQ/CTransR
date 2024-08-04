use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i64 = 1_000_000_007;

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    let r_sq = r * r % MOD;
    if b % 2 == 1 {
        return r_sq * a % MOD;
    }
    r_sq
}

fn check(n: usize, k: usize, hash: &[i64], base_pow: &[i64], s_hash_k: i64) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == s_hash_k * base_pow[i - 1] % MOD {
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

    let t: usize = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let l: usize = parts[1].parse().unwrap();
        let r: usize = parts[2].parse().unwrap();
        let s: Vec<i64> = ["0".to_string()].into_iter().chain(parts[3].chars().map(|c| c as i64 - 'a' as i64 + 1)).collect();

        let base: i64 = rng.gen_range(200..MOD);
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] * base + s[i]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(n, mid, &hash, &base_pow, hash[mid]);
            if yy >= l {
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
                    yy = check(n, mid, &hash, &base_pow, hash[mid]);
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