use rand::Rng;
use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u64 {
    rand::random::<u64>() * rand::random::<u64>()
}

fn rand2(l: u64, r: u64) -> u64 {
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

fn check(k: usize, hash: &[i64], base_pow: &[i64]) -> i64 {
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
    let t = io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut rng = rand::thread_rng();

    let base = rng.gen_range(200..MOD as u64);
    let base_pow = (0..=N).map(|i| base.pow(i as u32) % MOD).collect::<Vec<i64>>();

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: i64 = iter.next().unwrap().parse().unwrap();
        let r: i64 = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap();

        let hash = s
            .chars()
            .enumerate()
            .map(|(i, c)| base_pow[i] * c as i64 % MOD)
            .scan(0, |acc, x| {
                *acc = (*acc + x) % MOD;
                Some(*acc)
            })
            .collect::<Vec<i64>>();

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