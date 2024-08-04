use rand::Rng;
use std::io;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn main() {
    let mut rng = rand::thread_rng();
    let base: i64 = rng.gen_range(200..MOD as i64);

    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s: Vec<char> = iter.next().unwrap().chars().collect();

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base) % MOD;
            hash[i] = (hash[i - 1] + (s[i - 1] as i64) * base_pow[i] % MOD) % MOD) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &hash, &base_pow)) >= l {
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
                    if (yy = check(mid, n, &hash, &base_pow)) >= i {
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

fn check(k: usize, n: usize, hash: &[i64], base_pow: &[i64]) -> usize {
    let mut vo = 0;
    let k_hash = hash[k]; // Hash of the first k characters
    for i in 1..=n {
        if i + k - 1 <= n {
            let current_hash = (hash[i + k - 1] + MOD - hash[i - 1]) % MOD;
            if current_hash == (k_hash * base_pow[i - 1] % MOD) {
                vo += 1;
                // Skip to the next segment
                i += k - 1; // `i` incremented in next loop
            }
        }
    }
    vo
}