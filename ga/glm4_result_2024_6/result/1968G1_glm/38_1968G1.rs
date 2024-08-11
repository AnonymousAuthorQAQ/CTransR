use rand::Rng;
use std::time::SystemTime;

fn rand1() -> u32 {
    rand::random::<u32>() * rand::random::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r { return l; }
    rand1() % (r - l) + l
}

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn power(a: i64, b: i64) -> i64 {
    if b == 0 { return 1; }
    let r = power(a, b / 2);
    if b % 2 == 1 { r * r % MOD * a % MOD } else { r * r % MOD }
}

fn check(k: usize, hash: &[i64], base_pow: &[i64]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= N {
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
    let base: u32 = rng.gen_range(200..MOD as u32);

    let t = read_int();
    for _ in 0..t {
        let n = read_int();
        let l = read_int();
        let r = read_int();
        let s = read_str(n);

        let mut base_pow = vec![1; N + 1];
        let mut hash = vec![0; N + 1];
        let mut bp = 1;
        for i in 1..=n {
            bp = bp * base as i64 % MOD;
            base_pow[i] = bp;
            hash[i] = bp * s[i - 1] as i64 % MOD;
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

fn read_str(n: usize) -> Vec<char> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().chars().collect()
}