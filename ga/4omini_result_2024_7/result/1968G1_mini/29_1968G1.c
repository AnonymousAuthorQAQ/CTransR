use rand::{Rng, thread_rng};
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

fn check(n: usize, k: usize, hash: &[usize], base_pow: &[usize], s_len: usize) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        let expected = (hash[i + k - 1] + MOD - hash[i - 1]) % MOD;
        let actual = (hash[k] * base_pow[i - 1]) % MOD;
        if expected == actual {
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
    let mut rng = thread_rng();
    let mut t = String::new();

    stdin.lock().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let n = parts[0];
        let l = parts[1];
        let r = parts[2];

        let mut s = String::new();
        stdin.lock().read_line(&mut s).unwrap();
        let s = s.trim();

        let BASE = rng.gen_range(200..MOD) as usize;

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];

        base_pow[0] = 1;
        let mut bp = 1;
        
        for i in 1..=n {
            bp = bp * BASE % MOD;
            base_pow[i] = bp;
            hash[i] = (bp * s.chars().nth(i - 1).unwrap() as usize) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(n, mid, &hash, &base_pow, n)) >= l {
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
                    if (yy = check(n, mid, &hash, &base_pow, n)) >= i {
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