use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::{min, max};

const N: usize = 200005;
const MOD: i32 = 1000000007;

fn rand1() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>() * rng.gen::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let mut r = power(a, b / 2);
    if b & 1 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, n: usize, s: &Vec<char>, hash: &Vec<i32>, base_pow: &Vec<i32>) -> i32 {
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
    let mut base: i32 = rand2(200, MOD - 1);
    let mut base_pow: Vec<i32> = vec![1; N + 1];
    for i in 1..=N {
        base_pow[i] = base_pow[i - 1] * base % MOD;
    }

    let t: i32 = read_line().trim().parse().expect("Failed to read t");
    for _ in 0..t {
        let input: Vec<i32> = read_line()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse input"))
            .collect();
        let n = input[0] as usize;
        let l = input[1] as usize;
        let r = input[2] as usize;
        let s: Vec<char> = input[3].chars().collect();

        let mut hash: Vec<i32> = vec![0; N];
        for i in 1..=n {
            hash[i] = base_pow[i - 1] * s[i] as i32 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, n, &s, &hash, &base_pow) >= l as i32 {
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
                    if check(mid, n, &s, &hash, &base_pow) >= i as i32 {
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