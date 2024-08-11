use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    let r = r * r % MOD;
    if b % 2 == 1 {
        r * a % MOD
    } else {
        r
    }
}

fn check(s: &[i64], n: usize, k: usize, hash: &[i64], base_pow: &[i64]) -> i64 {
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
    let mut input = stdin.lock();
    
    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let params: Vec<usize> = buffer.split_whitespace()
                                         .map(|x| x.parse().unwrap())
                                         .collect();
        let n = params[0];
        let l = params[1];
        let r = params[2];

        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let s: Vec<char> = buffer.trim().chars().collect();

        let mut rng = rand::thread_rng();
        let base: i64 = rng.gen_range(200..MOD as i64);

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] + base_pow[i] * (s[i - 1] as i64)) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&s.iter().map(|&c| c as i64).collect::<Vec<i64>>(), n, mid, &hash, &base_pow)) >= l as i64 {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        print!("{} ", lower);

        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(&s.iter().map(|&c| c as i64).collect::<Vec<i64>>(), n, mid, &hash, &base_pow)) >= i as i64 {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{} ", lower);
        }

        println!();
    }
}