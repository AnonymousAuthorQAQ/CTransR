use rand::Rng;
use std::io::{self, BufRead};

fn random_base(l: u32, r: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(l..=r)
}

fn power(a: u64, b: u64, mod_val: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let mut r = power(a, b / 2, mod_val);
    r = (r * r) % mod_val;
    if b % 2 == 1 {
        r = (r * a) % mod_val;
    }
    r
}

fn check(n: usize, k: usize, hash: &[u64], base_pow: &[u64], mod_val: u64) -> usize {
    let mut vo = 0;
    let mut i = 0;

    while i + k <= n {
        let left_hash = (hash[i + k] + mod_val - hash[i]) % mod_val;
        let substring_hash = (hash[k] * base_pow[i]) % mod_val;
        
        if left_hash == substring_hash {
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
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    const N: usize = 200005;
    const MOD: u64 = 1_000_000_007;

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: Vec<char> = parts.next().unwrap().chars().collect();

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        let base = random_base(200, MOD as u32 - 1) as u64;

        for i in 0..n {
            base_pow[i + 1] = (base_pow[i] * base) % MOD;
            hash[i + 1] = (hash[i] + (s[i] as u64) * base_pow[i]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let count = check(n, mid, &hash, &base_pow, MOD);
            if count >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        let mut result: Vec<usize> = vec![lower];

        let mut yy = count;
        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let count = check(n, mid, &hash, &base_pow, MOD);
                    if count >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
                yy = count;
            }
            result.push(lower);
        }

        println!("{}", result.iter().map(usize::to_string).collect::<Vec<String>>().join(" "));
    }
}