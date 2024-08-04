use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    rng.gen::<u32>() * 1u32 * rng.gen::<u32>()
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
    let r = power(a, b/2);
    if b % 2 == 1 {
        return (r * r % MOD) * a % MOD;
    }
    return r * r % MOD;
}

fn check(hash: &Vec<i64>, base_pow: &Vec<i64>, k: usize) -> i32 {
    let mut vo = 0;
    let n = hash.len() - 1;
    for i in 1..=n-k+1 {
        if (hash[i+k-1] + MOD - hash[i-1]) % MOD == hash[k] * base_pow[i-1] % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let t: i32;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    t = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace().map(|x| x.parse().unwrap());
        let n = nums.next().unwrap();
        let l = nums.next().unwrap();
        let r = nums.next().unwrap();
        let s: String = nums.next().unwrap().parse().unwrap();
        
        let mut hash = vec![0; n+1];
        let mut base_pow = vec![0; n+1];
        
        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * BASE % MOD;
            base_pow[i] = bp * BASE % MOD;
            hash[i] = bp * s.chars().nth(i-1).unwrap() as i64 % MOD;
            hash[i] = (hash[i] + hash[i-1]) % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: i32;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, mid)) >= l {
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