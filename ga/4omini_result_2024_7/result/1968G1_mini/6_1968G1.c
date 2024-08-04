use rand::Rng;
use std::io::{self, BufRead};

const MOD: u64 = 1_000_000_007;
const N: usize = 200_005;

fn power(a: u64, mut b: u64) -> u64 {
    let mut result = 1;
    let mut a = a % MOD;

    while b > 0 {
        if b % 2 == 1 {
            result = (result * a) % MOD;
        }
        a = (a * a) % MOD;
        b /= 2;
    }
    result
}

fn check(hash: &[u64], base_pow: &[u64], n: usize, k: usize) -> usize {
    let mut vo = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1]) % MOD {
            vo += 1;
            i += k; // Move i forward by k
        } else {
            i += 1; // Move i forward by 1
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut rng = rand::thread_rng();
    let base: u64 = rng.gen_range(200..MOD);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();
        let s: Vec<u8> = input.trim().bytes().collect();

        let mut base_pow: Vec<u64> = vec![1; n + 1];
        let mut hash: Vec<u64> = vec![0; n + 1];

        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
            hash[i] = (hash[i - 1] * base + (s[i - 1] as u64)) % MOD; // s[i-1] to get 0-based index
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = check(&hash, &base_pow, n, mid);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        let mut yy = check(&hash, &base_pow, n, lower); // Initial yy after the first main check

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let yy = check(&hash, &base_pow, n, mid);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!(); // Print newline at end of test case
    }
}