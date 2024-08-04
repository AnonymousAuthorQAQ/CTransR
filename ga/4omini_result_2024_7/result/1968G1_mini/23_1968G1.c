use rand::Rng; // For generating random numbers
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let mut r = power(a, b / 2);
    r = (r * r) % MOD;
    if b % 2 == 1 {
        r = (r * a) % MOD;
    }
    r
}

fn check(k: usize, n: usize, hash: &[usize], base_pow: &[usize], s: &[usize]) -> usize {
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
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let t: usize = first_line.trim().parse().unwrap();

    for _ in 0..t {
        // Read inputs
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = parts[0];
        let l = parts[1];
        let r = parts[2];

        let s: Vec<usize> = lines.next().unwrap().unwrap().chars().map(|c| c as usize).collect();

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];
        let mut bp = 1;

        // Generate a random base
        let base: usize = rand::thread_rng().gen_range(200..MOD);

        base_pow[0] = 1;
        for i in 1..=n {
            bp = (bp * base) % MOD;
            base_pow[i] = bp;
            hash[i] = (bp * s[i] % MOD + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &hash, &base_pow, &s)) >= l {
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
                    if (yy = check(mid, n, &hash, &base_pow, &s)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!(); // Print newline at the end of each test case
    }
}