use rand::Rng;
use std::time::SystemTime;

fn rand1() -> u32 {
    rand::random::<u32>() * rand::random::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r { return l; }
    rand::random::<u32>() % (r - l) + l
}

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn power(a: u64, b: u64) -> u64 {
    if b == 0 { return 1; }
    let r = power(a, b / 2);
    if b % 2 == 1 { return r * r % MOD * a % MOD; }
    r * r % MOD
}

fn check(k: usize, hash: &[u64], base_pow: &[u64]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= hash.len() {
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
    let mut rng = rand::thread_rng();
    let base = rng.gen_range(200..MOD as u32);
    let mut base_pow = vec![0; N + 1];
    base_pow[0] = 1;

    let t = read_int();
    for _ in 0..t {
        let n = read_int();
        let l = read_int();
        let r = read_int();
        let s = read_string(n);

        let mut hash = vec![0; n + 1];
        let mut bp = 1;
        for i in 1..=n {
            bp = bp * base as u64 % MOD;
            base_pow[i] = bp;
            hash[i] = bp * s[i - 1] as u64 % MOD;
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

fn read_int() -> usize {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn read_string(n: usize) -> Vec<char> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.chars().collect()
}