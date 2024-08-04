use rand::{thread_rng, Rng};
use std::time::SystemTime;

fn rand1() -> u32 {
    let mut rng = thread_rng();
    rng.gen()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r { return l; }
    rand1() % (r - l) + l
}

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn power(a: u64, b: u64) -> u64 {
    if b == 0 { return 1; }
    let r = power(a, b / 2);
    if b & 1 == 1 { return r * r % MOD * a % MOD; }
    r * r % MOD
}

fn check(k: usize, hash: &[u64], base_pow: &[u64]) -> usize {
    let mut vo = 0;
    for i in 1..=N - k {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let t = 1; // Replace with actual input

    for _ in 0..t {
        let n = 10; // Replace with actual input
        let l = 2; // Replace with actual input
        let r = 5; // Replace with actual input
        let s = "abcabcabca"; // Replace with actual input

        let mut base_pow = vec![0; N + 1];
        base_pow[0] = 1;
        let mut hash = vec![0; N + 1];

        let mut bp = 1;
        for i in 1..=n {
            bp = bp * rand2(200, MOD as u32) as u64 % MOD;
            base_pow[i] = bp;
            hash[i] = bp * s.chars().nth(i - 1).unwrap() as u64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
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