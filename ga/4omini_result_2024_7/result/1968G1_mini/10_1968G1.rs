use rand::Rng;
use std::io::{self, Write};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    let mut res = (r * r) % MOD;
    if b % 2 == 1 {
        res = (res * a) % MOD;
    }
    res
}

fn check(k: usize, hash: &[i64], base_pow: &[i64], n: usize, s_hash: i64) -> usize {
    let mut vo = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        let hash_val = (hash[i + k - 1] + MOD - hash[i - 1]) % MOD;
        let expected_hash = (s_hash * base_pow[i - 1]) % MOD;

        if hash_val == expected_hash {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn rand2(l: u32, r: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(l..r)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s: Vec<char> = iter.next().unwrap().chars().collect();

        let base: i64 = rand2(200, MOD as u32 - 1) as i64;

        let mut base_pow: Vec<i64> = vec![1; n + 1];
        let mut hash: Vec<i64> = vec![0; n + 1];

        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
            hash[i] = (hash[i - 1] * base + s[i - 1] as i64) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;

            yy = check(mid, &hash, &base_pow, n, hash[mid]);
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
                    yy = check(mid, &hash, &base_pow, n, hash[mid]);
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