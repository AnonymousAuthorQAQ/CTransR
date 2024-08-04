use std::io::{self, Read};
use rand::Rng;

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let half = power(a, b / 2);
    let mut result = (half * half) % MOD;
    if b % 2 == 1 {
        result = (result * a) % MOD;
    }
    result
}

fn check(n: usize, k: usize, hash: &[usize], base_pow: &[usize]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        let h1 = (hash[i + k - 1] + MOD - hash[i - 1]) % MOD;
        let h2 = (hash[k] * base_pow[i - 1]) % MOD;
        if h1 == h2 {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s: Vec<char> = iter.next().unwrap().chars().collect();

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let mut base = rand::thread_rng().gen_range(200..MOD);

        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
            hash[i] = (hash[i - 1] * base + s[i - 1] as usize) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(n, mid, &hash, &base_pow);
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
                    yy = check(n, mid, &hash, &base_pow);
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