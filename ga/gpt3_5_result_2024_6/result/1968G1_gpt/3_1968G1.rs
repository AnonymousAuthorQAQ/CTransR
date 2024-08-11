use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn main() {
    let mut s = String::new();
    let mut hash = vec![0; N];
    let mut base_pow = vec![0; N + 1];
    let mut n = 0;
    let mut l = 0;
    let mut r = 0;

    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed.into());

    let mut base: u64 = rng.gen_range(200, MOD - 1) as u64;

    let t: usize = input();

    for _ in 0..t {
        n = input();
        l = input();
        r = input();
        s = input_string();

        let mut bp: u64 = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp.wrapping_mul(base) % MOD;
            base_pow[i] = bp.wrapping_mul(base) % MOD;
            hash[i] = bp.wrapping_mul(s.chars().nth(i - 1).unwrap() as u64) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, mid)) >= l {
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
                    if (yy = check(&hash, &base_pow, mid)) >= i {
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

fn check(hash: &Vec<u64>, base_pow: &Vec<u64>, k: usize) -> usize {
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

fn input() -> usize {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("Error reading input");
    let n: usize = input_text.trim().parse().unwrap();
    n
}

fn input_string() -> String {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("Error reading input");
    input_text.trim().to_string()
}

fn print<T: std::string::ToString>(s: T) {
    print!("{}", s.to_string());
}

fn println<T: std::string::ToString>(s: T) {
    println!("{}", s.to_string());
}