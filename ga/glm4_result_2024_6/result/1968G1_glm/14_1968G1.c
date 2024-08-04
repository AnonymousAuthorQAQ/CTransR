use std::io;
use rand::Rng;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    rand::random::<i64>() * 1u64.wrapping_shl(32) as i64
}

fn rand2(l: i64, r: i64) -> i64 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: i64, hash: &Vec<i64>, base_pow: &Vec<i64>, s: &[i64]) -> i64 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= s.len() as i64 {
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
    let base = rng.gen_range(200..MOD);
    let mut base_pow = vec![0; N + 1];
    base_pow[0] = 1;
    for i in 1..=N {
        base_pow[i] = base_pow[i - 1] * base % MOD;
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: i64 = line.trim().parse().unwrap();

    while t > 0 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: i64 = parts.next().unwrap().parse().unwrap();
        let l: i64 = parts.next().unwrap().parse().unwrap();
        let r: i64 = parts.next().unwrap().parse().unwrap();
        let s: Vec<i64> = parts.next().unwrap().chars().map(|c| c as i64).collect();

        let mut hash = vec![0; N];
        let mut bp = 1;
        for i in 1..=n as usize {
            hash[i] = bp * s[i - 1] % MOD;
            if i > 1 {
                hash[i] = (hash[i] + hash[i - 1]) % MOD;
            }
            bp = bp * base % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow, &s) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i > check(lower, &hash, &base_pow, &s) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow, &s) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            println!("{}", lower);
        }

        t -= 1;
    }
}