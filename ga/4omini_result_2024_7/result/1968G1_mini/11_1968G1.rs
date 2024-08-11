use rand::Rng;
use std::cmp::min;
use std::io::{self, Write};

const MOD: usize = 1_000_000_007;
const N: usize = 200_005;

struct TestCase {
    n: usize,
    l: usize,
    r: usize,
    s: String,
}

fn power(base: usize, exp: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let mut res = power(base, exp / 2);
    res = (res * res) % MOD;
    if exp % 2 == 1 {
        res = (res * base) % MOD;
    }
    res
}

fn build_hash_and_base(s: &str, n: usize, base: usize) -> (Vec<usize>, Vec<usize>) {
    let mut base_pow = vec![1; n + 1];
    let mut hash: Vec<usize> = vec![0; n + 1];

    for i in 1..=n {
        base_pow[i] = (base_pow[i - 1] * base) % MOD;
        hash[i] = (hash[i - 1] + s.as_bytes()[i - 1] as usize * base_pow[i]) % MOD;
    }

    (hash, base_pow)
}

fn check(hash: &[usize], base_pow: &[usize], n: usize, k: usize) -> usize {
    let mut occurrences = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD ==
            (hash[k] * base_pow[i - 1]) % MOD {
            occurrences += 1;
            i += k;
        } else {
            i += 1;
        }
    }

    occurrences
}

fn main() {
    let mut rng = rand::thread_rng();
    let base = rng.gen_range(200..MOD);

    let mut stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    let mut output = String::new();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let nums: Vec<usize> = input.split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = nums[0];
        let l = nums[1];
        let r = nums[2];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let s = input.trim().to_string();

        let (hash, base_pow) = build_hash_and_base(&s, n, base);

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = (lower + upper) / 2;
            let occurrences = check(&hash, &base_pow, n, mid);
            if occurrences >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        output.push_str(&lower.to_string());

        // Further check for lengths greater than 'l'
        let mut yy = check(&hash, &base_pow, n, lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = (lower + upper) / 2;
                    let occurrences = check(&hash, &base_pow, n, mid);
                    if occurrences >= i {
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