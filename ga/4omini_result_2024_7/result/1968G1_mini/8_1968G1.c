use rand::Rng;
use std::io::{self, BufRead};

const MOD: usize = 1_000_000_007;
const N: usize = 200_005;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    let r = (r * r) % MOD;
    if b % 2 == 1 {
        (r * a) % MOD
    } else {
        r
    }
}

fn check(k: usize, n: usize, hash: &[usize], base_pow: &[usize], s_hash: usize) -> usize {
    let mut vo = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (s_hash * base_pow[i - 1]) % MOD {
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
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut rng = rand::thread_rng();
    let base: usize = rng.gen_range(200..MOD - 1);

    let mut lines = input.split_whitespace();

    let t: usize = lines.next().unwrap().parse().unwrap();
    let mut results = Vec::new();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let l: usize = lines.next().unwrap().parse().unwrap();
        let r: usize = lines.next().unwrap().parse().unwrap();
        let s: Vec<char> = lines.next().unwrap().chars().collect();

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let mut bp = 1;

        for i in 1..=n {
            bp = (bp * base) % MOD;
            base_pow[i] = bp;
            hash[i] = (hash[i - 1] + (bp * (s[i - 1] as usize)) % MOD) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(mid, n, &hash, &base_pow, hash[mid]);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        results.push(lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(mid, n, &hash, &base_pow, hash[mid]);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            results.push(lower);
        }
    }

    println!("{}", results.iter().map(usize::to_string).collect::<Vec<String>>().join(""));
}