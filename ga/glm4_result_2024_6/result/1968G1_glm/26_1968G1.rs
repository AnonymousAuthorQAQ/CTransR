use rand::{thread_rng, Rng};
use std::time::SystemTime;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let mut rng = thread_rng();
    rng.gen()
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, hash: &[u64], base_pow: &[u64]) -> usize {
    let mut vo = 0;
    for i in 1..=N - k + 1 {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let mut rng = thread_rng();
    let base: u64 = rng.gen_range(200..MOD);
    let mut base_pow = vec![0; N + 1];
    base_pow[0] = 1;
    for i in 1..=N {
        base_pow[i] = base_pow[i - 1] * base % MOD;
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let l: usize = tokens.next().unwrap().parse().unwrap();
        let r: usize = tokens.next().unwrap().parse().unwrap();
        let s = tokens.next().unwrap();

        let mut hash = vec![0; N];
        for (i, c) in s.chars().enumerate() {
            hash[i + 1] = hash[i] + (base_pow[i] * c as u64 % MOD) % MOD;
            hash[i + 1] %= MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i > check(lower, &hash, &base_pow) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            println!("{}", lower);
        }
    }
}