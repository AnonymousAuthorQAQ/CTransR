use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    rand::random::<u32>() * 1 * rand::random::<u32>()
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
        return ((r * r % MOD) * a % MOD) % MOD;
    }
    (r * r % MOD) % MOD
}

fn check(k: usize, base_pow: &Vec<i64>, hash: &Vec<i64>) -> i32 {
    let mut vo = 0;
    for i in 1..=hash.len() - k {
        if hash[i + k - 1].rem_euclid(MOD) == (hash[i - 1].rem_euclid(MOD) + MOD - hash[k] * base_pow[i - 1] % MOD) % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let t: usize = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input_str: String = stdin.lock().lines().next().unwrap().unwrap().trim().to_string();
        let input: Vec<i32> = input_str.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = input[0] as usize;
        let l = input[1] as usize;
        let r = input[2] as usize;
        let s = input[3].clone();

        let mut base_pow = vec![0; n + 1];
        let BASE = rand2(200, MOD as u32 - 1);
        let mut bp = 1;
        for i in 1..=n {
            base_pow[i] = bp * BASE % MOD;
            bp = bp * BASE % MOD;
        }

        let mut hash = vec![0; n + 1];
        for i in 1..=n {
            hash[i] = bp * s[i as usize] as i64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: usize = 0;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let result = check(mid, &base_pow, &hash);
            if result >= l as i32 {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let result = check(mid, &base_pow, &hash);
                    if result >= i as i32 {
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