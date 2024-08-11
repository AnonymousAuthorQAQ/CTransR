use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32;
    return now.wrapping_mul(now);
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    return rand1() % (r - l) + l;
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(k: usize, n: usize, hash: &Vec<i64>, base_pow: &Vec<i64>) -> i32 {
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
    return vo;
}

fn main() {
    let t: i32 = 1; // Set the number of test cases here

    let mut input_data: Vec<(i32, i32, i32, String)> = Vec::new();
    input_data.push((5, 2, 4, String::from("abcde"))); // Add test case data here

    let mut base_pow: Vec<i64> = vec![1; N + 1];
    let mut hash: Vec<i64> = vec![0; N];
    let mut s: Vec<char> = Vec::new();

    for _ in 0..t {
        let (n, l, r, input_str) = input_data.remove(0);
        let s: Vec<char> = input_str.chars().collect();

        let mut BASE = rand2(200, MOD as u32 - 1);

        let mut bp = 1;
        for i in 1..=n {
            bp = bp * BASE % MOD;
            base_pow[i] = bp;
            hash[i] = bp * s[i - 1] as i64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &hash, &base_pow)) >= l {
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
                    if (yy = check(mid, n, &hash, &base_pow)) >= i {
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