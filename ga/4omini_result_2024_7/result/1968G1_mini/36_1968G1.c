use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let mut r = power(a, b / 2);
    r = r * r % MOD;
    if b % 2 == 1 {
        r = r * a % MOD;
    }
    r
}

fn check(hash: &[usize], base_pow: &[usize], k: usize, n: usize, l: usize) -> usize {
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
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (n, l, r) = (first_line[0], first_line[1], first_line[2]);

        let s: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        let mut rng = rand::thread_rng();
        let base = rng.gen_range(200..MOD);

        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] + s[i - 1] as usize * base_pow[i]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = check(&hash, &base_pow, mid, n, l);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        let mut yy = check(&hash, &base_pow, lower, n, l);
        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let yy_new = check(&hash, &base_pow, mid, n, l);
                    if yy_new >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!(" {}", lower);
            yy = check(&hash, &base_pow, lower, n, l);
        }
        println!();
    }
}