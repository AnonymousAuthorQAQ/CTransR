use std::time::{SystemTime, UNIX_EPOCH};

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

const N: usize = 200005;
const MOD: u32 = 1000000007;

fn power(a: u32, b: u32) -> u32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(hash: &Vec<u32>, base_pow: &Vec<u32>, n: usize, k: usize) -> i32 {
    let mut vo = 0;
    for i in 1..=n-k+1 {
        if (hash[i+k-1] + MOD - hash[i-1]) % MOD == hash[k] * base_pow[i-1] % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let BASE: u32 = rand2(200, MOD-1);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let splits: Vec<u32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = splits[0] as usize;
        let l = splits[1] as usize;
        let r = splits[2] as usize;
        let s = splits[3].chars().collect::<Vec<char>>();

        let mut base_pow: Vec<u32> = vec![1; n+1];
        for i in 1..=n {
            base_pow[i] = base_pow[i-1] * BASE % MOD;
        }

        let mut hash: Vec<u32> = vec![0; n+1];
        for i in 1..=n {
            hash[i] = hash[i-1] * BASE % MOD + s[i-1] as u32 % MOD;
            hash[i] %= MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, n, mid)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        for i in l+1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(&hash, &base_pow, n, mid)) >= i {
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