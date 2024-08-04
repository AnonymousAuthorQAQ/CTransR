use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1000000007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    let mut result = r * r % MOD;
    if b % 2 == 1 {
        result = result * a % MOD;
    }
    result
}

fn check(n: usize, k: usize, hash: &[usize], base_pow: &[usize]) -> usize {
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
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    // Read number of test cases
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
        let s: Vec<char> = input.trim().chars().collect();

        // Generate a random base
        let mut rng = rand::thread_rng();
        let base: usize = rng.gen_range(200..MOD);

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] * base + (s[i - 1] as usize)) % MOD; // Convert char to usize
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = check(n, mid, &hash, &base_pow);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        // Output for the first value
        print!("{}", lower);

        let mut yy = check(n, lower, &hash, &base_pow);
        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let yy_new = check(n, mid, &hash, &base_pow);
                    if yy_new >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }
        println!(); // Newline at the end of each test case
    }
}