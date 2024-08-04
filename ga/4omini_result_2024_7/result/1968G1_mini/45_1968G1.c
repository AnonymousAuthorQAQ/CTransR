use rand::Rng; // For random number generation
use std::io::{self, BufRead}; // For input handling
use std::cmp; // For comparison operations

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..t {
        let input = lines.next().unwrap().unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: String = parts.next().unwrap().to_string();

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];

        let base = rng.gen_range(200..MOD);

        base_pow[0] = 1;
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] + base_pow[i] * s.chars().nth(i - 1).unwrap() as usize) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = check(mid, n, &hash, &base_pow, &s);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{}", lower);

        for i in (l + 1)..=r {
            if i > check(lower, n, &hash, &base_pow, &s) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let yy = check(mid, n, &hash, &base_pow, &s);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }
        println!(); // Newline after each test case
    }
}

fn check(k: usize, n: usize, hash: &[usize], base_pow: &[usize], s: &String) -> usize {
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