use std::io::{self, BufRead};
use rand::Rng;

const N: usize = 200005;
const MOD: u64 = 1_000_000_007;

fn power(a: u64, b: u64) -> u64 {
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

fn check(n: usize, k: usize, hash: &[u64], base_pow: &[u64], s_hash_k: u64) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (s_hash_k * base_pow[i - 1]) % MOD {
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
    let mut reader = stdin.lock();
    let mut rng = rand::thread_rng();

    let t: usize = {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    for _ in 0..t {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let l = parts[1];
        let r = parts[2];

        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let s: Vec<char> = s.trim().chars().collect();
        
        let base = rng.gen_range(200..MOD as usize);
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base as u64) % MOD;
            hash[i] = (hash[i - 1] + base_pow[i] * (s[i - 1] as u64)) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let s_hash_k = (base_pow[mid] * (s[0] as u64)) % MOD; // Hash for the first k characters
            let yy = check(n, mid, &hash, &base_pow, s_hash_k);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{}", lower);

        let mut yy = check(n, lower, &hash, &base_pow, (base_pow[lower] * (s[0] as u64)) % MOD);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let s_hash_k = (base_pow[mid] * (s[0] as u64)) % MOD; // Hash for the first k characters
                    yy = check(n, mid, &hash, &base_pow, s_hash_k);
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