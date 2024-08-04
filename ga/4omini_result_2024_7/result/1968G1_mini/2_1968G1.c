use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(hash: &[usize], base_pow: &[usize], n: usize, k: usize) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        let expected_hash = (hash[k] * base_pow[i - 1]) % MOD;
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == expected_hash {
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
    let base: usize = rng.gen_range(200..MOD as usize);

    let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = reader.lines().next().unwrap().unwrap();
        let mut params = line.split_whitespace();
        let n: usize = params.next().unwrap().parse().unwrap();
        let l: usize = params.next().unwrap().parse().unwrap();
        let r: usize = params.next().unwrap().parse().unwrap();
        let s: Vec<char> = params.next().unwrap().chars().collect();

        // Initialize base powers and hash
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = ((base_pow[i] * (s[i - 1] as usize)) % MOD + hash[i - 1]) % MOD);
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(&hash, &base_pow, n, mid);
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
                    yy = check(&hash, &base_pow, n, mid);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!(" {}", lower);
        }
        println!();
    }
}