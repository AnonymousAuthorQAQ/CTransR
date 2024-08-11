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
    let mut rng = rand::thread_rng();
    let base = rng.gen_range(200..MOD as u32);
    
    let t = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse::<i32>().expect("Invalid input")
    };

    for _ in 0..t {
        let n = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().parse::<i32>().expect("Invalid input")
        };
        let l = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().parse::<i32>().expect("Invalid input")
        };
        let r = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().parse::<i32>().expect("Invalid input")
        };
        let s = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().to_string()
        };

        let mut bp = 1;
        let mut base_pow = vec![0; n as usize + 1];
        let mut hash = vec![0; n as usize + 1];

        base_pow[0] = 1;
        for i in 1..=n as usize {
            bp = bp * base as i64 % MOD;
            base_pow[i] = bp;
            hash[i] = bp * s.as_bytes()[i - 1] as i64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n as usize + 1;
        let mut yy;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, &hash, &base_pow)) >= l as usize {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l as usize + 1..=r as usize {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(mid, &hash, &base_pow)) >= i {
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