use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    rand::random::<u32>() * 1u32 * rand::random::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
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
    vo
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let l: usize = parts[1].parse().unwrap();
        let r: usize = parts[2].parse().unwrap();
        let s: String = parts[3].to_string();

        let mut hash: Vec<i64> = vec![0; N + 2];
        let mut base_pow: Vec<i64> = vec![0; N + 1];
        let mut BASE = rand2(200, MOD as u32 - 1);

        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * BASE as i64 % MOD;
            base_pow[i] = bp * BASE as i64 % MOD;
            hash[i] = bp * s.chars().nth(i - 1).unwrap() as i64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: i32;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &hash, &base_pow)) >= l as i32 {
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
                    if (yy = check(mid, n, &hash, &base_pow)) >= i as i32 {
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