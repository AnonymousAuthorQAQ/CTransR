use rand::Rng;
use std::io::{self, BufRead};

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

fn check(k: usize, n: usize, hash: &[usize], base_pow: &[usize], s: &[usize]) -> usize {
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
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut rng = rand::thread_rng();
    let base: usize = rng.gen_range(200..MOD); // Random base in the range [200, MOD-1]

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = first_line[0];
        let l = first_line[1];
        let r = first_line[2];

        let s: Vec<usize> = lines.next().unwrap().unwrap()
            .chars()
            .map(|c| (c as usize) % MOD) // Convert character to usize value
            .collect();

        let mut bp = 1;
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow[i] = bp;
            hash[i] = (bp * s[i] % MOD + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

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

        println!();
    }
}