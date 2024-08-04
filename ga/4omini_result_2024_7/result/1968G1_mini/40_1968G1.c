use std::io::{self, Write};
use rand::Rng;

const N: usize = 200005;
const MOD: i32 = 1_000_000_007;

#[derive(Default)]
struct TestCase {
    n: usize,
    l: usize,
    r: usize,
    s: Vec<u8>,
}

fn power(a: i32, mut b: i32) -> i32 {
    let mut result = 1;
    let mut a = a;
    
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % MOD;
        }
        a = a * a % MOD;
        b /= 2;
    }
    
    result
}

fn check(n: usize, k: usize, hash: &[i32], base_pow: &[i32; N + 1], s_hash: i32) -> i32 {
    let mut count = 0;
    let mut i = 0;

    while i + k - 1 < n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (s_hash * base_pow[i] % MOD) {
            count += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    
    count
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    let mut rng = rand::thread_rng();
    let base: i32 = rng.gen_range(200..MOD);

    let mut output = String::new();

    for _ in 0..t {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = parts[0];
        let l = parts[1];
        let r = parts[2];

        let mut s: Vec<u8> = vec![0; n + 2];
        stdin.read_line(&mut input).unwrap();
        input.bytes().take(n).enumerate().for_each(|(i, byte)| s[i + 1] = byte);

        // Creating hash and base power array
        let mut hash = vec![0; n + 1];
        let mut base_pow = [1; N + 1];
        let mut bp = 1;
        
        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow[i] = bp;
            hash[i] = (hash[i - 1] * base + s[i] as i32) % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;

        // Binary search for the largest k such that check(k) >= l
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let result = check(n, mid, &hash, &base_pow, hash[mid]);
            if result >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        output.push_str(&lower.to_string());
        
        let mut yy = check(n, lower, &hash, &base_pow, hash[lower]);
        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;

                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(n, mid, &hash, &base_pow, hash[mid]);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            output.push_str(&lower.to_string());
        }
        output.push('\n');
    }

    print!("{}", output);
}