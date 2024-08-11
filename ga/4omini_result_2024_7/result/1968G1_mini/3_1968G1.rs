use rand::Rng;
use std::io::{self, BufRead};

const MOD: usize = 1_000_000_007;
const N: usize = 200_005;

fn power(a: usize, mut b: usize) -> usize {
    let mut result = 1;
    let mut base = a % MOD;

    while b > 0 {
        if b % 2 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        b /= 2;
    }
    result
}

fn check(n: usize, k: usize, s: &Vec<usize>, hash: &Vec<usize>, base_pow: &Vec<usize>) -> usize {
    let mut vo = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1]) % MOD {
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

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        // Read n, l, r, and s
        let inputs: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = inputs[0];
        let l = inputs[1];
        let r = inputs[2];

        let s: Vec<usize> = lines.next().unwrap().unwrap().chars()
            .map(|c| c as usize - 'a' as usize + 1)  // Convert char to 1-based indexing (assuming 'a' to 'z')
            .collect();

        let mut base_pow = vec![0; n + 1];
        let mut hash = vec![0; n + 1];
        let mut rand = rand::thread_rng();
        let base: usize = rand.gen_range(200..MOD-1);

        base_pow[0] = 1;
        let mut bp = 1;

        for i in 1..=n {
            bp = (bp * base) % MOD;
            base_pow[i] = bp;
            hash[i] = (hash[i - 1] + bp * s[i - 1]) % MOD; // s[i - 1] because s is 0-indexed in Rust
        }

        let mut low = 0;
        let mut high = n + 1;

        while high - low > 1 {
            let mid = (low + high) / 2;
            let yy = check(n, mid, &s, &hash, &base_pow);
            if yy >= l {
                low = mid;
            } else {
                high = mid;
            }
        }

        print!("{}", low);

        let mut yy = check(n, low, &s, &hash, &base_pow);
        for i in l + 1..=r {
            if i > yy {
                high = low + 1;
                low = 0;
                while high - low > 1 {
                    let mid = (low + high) / 2;
                    let yy_new = check(n, mid, &s, &hash, &base_pow);
                    if yy_new >= i {
                        low = mid;
                    } else {
                        high = mid;
                    }
                }
                yy = check(n, low, &s, &hash, &base_pow);
            }
            print!("{}", low);
        }
        println!();
    }
}