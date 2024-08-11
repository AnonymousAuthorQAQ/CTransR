use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let random_number = timestamp.wrapping_mul(timestamp);
    random_number
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
    let mut result = power(a, b / 2);
    if b % 2 == 1 {
        return result.wrapping_mul(result).wrapping_mul(a) % MOD;
    }
    result.wrapping_mul(result) % MOD
}

fn check(k: usize, n: usize, s: &str, base_pow: &Vec<u64>, hash: &Vec<u64>) -> usize {
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
    let mut base_pow: Vec<u64> = vec![1; N + 1];
    let mut hash: Vec<u64> = vec![0; N];
    
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap().to_string();

        let mut BASE = rand2(200, MOD - 1);
        let mut bp = 1;
        for i in 1..=n {
            bp = bp.wrapping_mul(BASE) % MOD;
            base_pow[i] = bp.wrapping_mul(BASE) % MOD;
            hash[i] = bp.wrapping_mul(s.chars().nth(i-1).unwrap() as u64) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &s, &base_pow, &hash)) >= l {
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
                    if (yy = check(mid, n, &s, &base_pow, &hash)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!();
    }
}