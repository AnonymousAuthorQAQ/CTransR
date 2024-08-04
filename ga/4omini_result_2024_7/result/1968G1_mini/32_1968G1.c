use rand::Rng;
use std::cmp::min;

const N: usize = 200005;
const MOD: i32 = 1_000_000_007;

fn rand2(l: u32, r: u32) -> u32 {
    let mut rng = rand::thread_rng();
    if l == r {
        return l;
    }
    rng.gen_range(l..r)
}

fn power(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    let r = (r as i64 * r as i64 % MOD as i64) as i32;
    if b % 2 == 1 {
        return (r as i64 * a as i64 % MOD as i64) as i32;
    }
    r
}

fn check(hash: &[i32], base_pow: &[i32], k: usize, n: usize, s: &[u8]) -> usize {
    let mut vo = 0;
    for i in 1..=n - k + 1 {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] as i64 * base_pow[i - 1] as i64 % MOD as i64) as i32 {
            vo += 1;
            i += k - 1; // Jump `k`
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let BASE = rand2(200, MOD as u32 - 1) as i32;

    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Failed to read line");
    let t: usize = t.trim().parse().expect("Not a number");

    for _ in 0..t {
        let mut parameters = String::new();
        std::io::stdin().read_line(&mut parameters).expect("Failed to read line");
        let params: Vec<usize> = parameters
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number"))
            .collect();
        
        let n = params[0];
        let l = params[1];
        let r = params[2];

        let mut s = vec![0; N + 2];
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Failed to read line");
        for (i, c) in buf.trim().chars().enumerate() {
            s[i + 1] = c as i32; // Cast to i32 for consistency with hash array
        }

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] as i64 * BASE as i64 % MOD as i64) as i32;
            hash[i] = (base_pow[i] as i64 * s[i] as i64 % MOD as i64) as i32;
            hash[i] = (hash[i] + hash[i - 1] + MOD) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, mid, n, &s)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(&hash, &base_pow, mid, n, &s)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }
        println!(); // Print newline at the end of each test case
    }
}