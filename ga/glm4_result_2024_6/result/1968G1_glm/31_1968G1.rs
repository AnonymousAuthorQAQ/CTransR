use std::io::{self, BufRead};

const MOD: i64 = 1000000007;

fn rand1() -> i64 {
    rand::random::<i64>() * 1u64.wrapping_shl(32) as i64
}

fn rand2(l: i64, r: i64) -> i64 {
    if l == r { return l; }
    rand1() % (r - l) + l
}

const N: usize = 200005;

fn power(a: i64, b: i64) -> i64 {
    if b == 0 { return 1; }
    let r = power(a, b / 2);
    if b & 1 == 1 { return r * r % MOD * a % MOD; }
    r * r % MOD
}

fn check(k: i64, hash: &[i64], base_pow: &[i64]) -> i64 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= hash.len() as i64 {
        if (hash[(i + k - 1) as usize] + MOD - hash[(i - 1) as usize]) % MOD == hash[k as usize] * base_pow[(i - 1) as usize] % MOD {
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
    let mut lines = stdin.lock().lines();
    let t: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    while t > 0 {
        let mut s = lines.next().unwrap().unwrap();
        let n: i64 = s.parse().unwrap();
        let l: i64 = s.parse().unwrap();
        let r: i64 = s.parse().unwrap();
        s = lines.next().unwrap().unwrap();
        let mut hash = vec![0; (n + 1) as usize];
        let mut base_pow = vec![0; (n + 1) as usize];
        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * rand2(200, MOD - 1) % MOD;
            base_pow[i as usize] = bp;
            hash[i as usize] = bp * s.as_bytes()[i as usize] as i64 % MOD;
            hash[i as usize] = (hash[i as usize] + hash[(i - 1) as usize]) % MOD;
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
        println!();
        t -= 1;
    }
}