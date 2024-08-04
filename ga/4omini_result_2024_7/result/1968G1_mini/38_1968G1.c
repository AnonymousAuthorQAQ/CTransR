use std::io::{self, BufRead};
use rand::Rng;  // Import the RNG functionality from the rand crate

const N: usize = 200005;
const MOD: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        // Read n, l, r, and s
        let input: Vec<_> = lines.next().unwrap().unwrap().split_whitespace().collect();
        let n: usize = input[0].parse().unwrap();
        let l: usize = input[1].parse().unwrap();
        let r: usize = input[2].parse().unwrap();
        let s: Vec<char> = format!("0{}", input[3]).chars().collect(); // Add extra index for easier handling (1-indexed)

        let base = rand::thread_rng().gen_range(200..MOD as u64) as i64;

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let mut bp: i64 = 1;
        
        for i in 1..=n {
            bp = (bp * base) % MOD;
            base_pow[i] = bp;
            hash[i] = (hash[i - 1] + (s[i] as i64 * base_pow[i]) % MOD) % MOD;
        }

        let (mut lower, mut upper) = (0, n + 1);
        let mut yy: i64;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(mid, n, &hash, &base_pow);
            if yy >= l as i64 {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{} ", lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;

                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(mid, n, &hash, &base_pow);
                    if yy >= i as i64 {
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

fn check(k: usize, n: usize, hash: &Vec<i64>, base_pow: &Vec<i64>) -> i64 {
    let mut vo = 0;
    let mut i = 1;
    
    while i + k - 1 <= n {
        let h1 = (hash[i + k - 1] + MOD - hash[i - 1]) % MOD;
        let h2 = (hash[k] * base_pow[i - 1]) % MOD;
        if h1 == h2 {
            vo += 1;
            i += k;  // Non-overlapping
        } else {
            i += 1;  // Overlapping
        }
    }
    vo
}