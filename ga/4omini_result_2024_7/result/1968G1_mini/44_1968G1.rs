use rand::Rng;
use std::io;

const N: usize = 200005;
const MOD: i64 = 1_000_000_007;

fn rand1() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..u32::MAX) * rng.gen_range(0..u32::MAX)
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        l
    } else {
        rand1() % (r - l) + l
    }
}

fn power(a: i64, b: i64) -> i64 {
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

fn check(k: usize, n: usize, hash: &[i64], base_pow: &[i64]) -> usize {
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
    let mut rng = rand::thread_rng();
    let base = rand2(200, (MOD as u32) - 1);
    
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: String = parts.next().unwrap().to_string();

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        for i in 1..=n {
            base_pow[i] = (base_pow[i - 1] * base as i64) % MOD;
            hash[i] = (hash[i - 1] + (s.chars().nth(i - 1).unwrap() as i64)) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(mid, n, &hash, &base_pow);
            if yy >= l {
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
                    yy = check(mid, n, &hash, &base_pow);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!(); // Print newline after each test case
    }
}