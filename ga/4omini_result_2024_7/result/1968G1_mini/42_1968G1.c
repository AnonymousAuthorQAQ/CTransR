use std::io::{self, BufRead};
use rand::Rng;

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let mut r = power(a, b / 2);
    r = r * r % MOD;
    if b % 2 == 1 {
        r = r * a % MOD;
    }
    r
}

fn check(k: usize, n: usize, hash: &[usize], base_pow: &[usize], s_hash: usize) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (s_hash * base_pow[i - 1] % MOD) {
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
    let base: usize = rng.gen_range(200..MOD as usize);

    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s: Vec<char> = iter.next().unwrap().chars().collect();

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let mut s_hash = 0;

        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            s_hash = (s_hash * base + (s[i - 1] as usize)) % MOD;
            hash[i] = (hash[i - 1] + s_hash * base_pow[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(mid, n, &hash, &base_pow, s_hash);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{} ", lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(mid, n, &hash, &base_pow, s_hash);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{} ", lower);
        }

        println!();
    }
}