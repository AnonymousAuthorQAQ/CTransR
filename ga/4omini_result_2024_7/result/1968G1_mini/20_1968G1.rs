use std::io::{self, BufRead};
use rand::Rng;

const N: usize = 200005;
const MOD: usize = 1000000007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return (r * r % MOD) * a % MOD;
    }
    return r * r % MOD;
}

fn check(k: usize, n: usize, hash: &[usize], base_pow: &[usize], s: &[usize]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1] % MOD) {
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
    
    // Randomly generate a base
    let mut rng = rand::thread_rng();
    let base: usize = rng.gen_range(200..MOD as usize);

    let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = reader.lines().next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: Vec<usize> = format!(" {}", parts.next().unwrap()).chars().map(|c| c as usize - 'a' as usize + 1).collect(); // Convert characters into corresponding integers

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];
        base_pow[0] = 1;

        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
            hash[i] = (hash[i - 1] + s[i] * base_pow[i - 1]) % MOD;
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

        let mut yy = check(lower, n, &hash, &base_pow, &s);
        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(mid, n, &hash, &base_pow, &s);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!(" {}", lower);
        }
        println!(); // New line at the end of each test case output
    }
}