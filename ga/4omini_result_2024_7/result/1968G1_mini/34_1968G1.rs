use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i32 = 1_000_000_007;

fn power(a: i32, mut b: i32) -> i32 {
    let mut result = 1;
    let mut base = a;
    while b > 0 {
        if b % 2 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        b /= 2;
    }
    result
}

fn check(k: usize, n: usize, hashes: &Vec<i32>, base_pows: &Vec<i32>, s: &Vec<i32>) -> i32 {
    let mut vo = 0;
    let mut i = 0;
    while i + k <= n {
        if (hashes[i + k - 1] + MOD - hashes[i - 1]) % MOD == (hashes[k - 1] * base_pows[i]) % MOD {
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

    let mut rng = rand::thread_rng();
    let base: i32 = rng.gen_range(200..MOD); // Generate a random base in the specified range

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let n = parts[0];
        let l = parts[1];
        let r = parts[2];
        
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s: Vec<i32> = input.trim().chars().map(|c| c as i32 - 'a' as i32 + 1).collect();

        let mut base_pows = vec![1; n + 1];
        let mut hashes = vec![0; n + 1];

        for i in 1..=n {
            base_pows[i] = base_pows[i - 1] * base % MOD;
            hashes[i] = (hashes[i - 1] + base_pows[i - 1] * s[i - 1]) % MOD; // s is 0-indexed
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = (lower + upper) / 2;
            let yy = check(mid, n, &hashes, &base_pows, &s);
            if yy >= l as i32 {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        let mut yy = check(lower, n, &hashes, &base_pows, &s);

        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;

                while upper - lower > 1 {
                    let mid = (lower + upper) / 2;
                    let yy = check(mid, n, &hashes, &base_pows, &s);
                    if yy >= i as i32 {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }
        println!(); // Print a newline at the end of each test case
    }
}