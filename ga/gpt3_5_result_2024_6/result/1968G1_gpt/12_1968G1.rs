use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const N: usize = 200005;
const MOD: i32 = 1000000007;

fn rand1() -> u32 {
    return rand::thread_rng().gen::<u32>() * 1 * rand::thread_rng().gen::<u32>();
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    return rand1() % (r - l) + l;
}

fn power(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 == 1 {
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(k: usize, n: usize, s: Vec<char>, base_pow: Vec<i32>, hash: Vec<i32>) -> i32 {
    let mut vo = 0;
    for i in 1..=n {
        if i + k - 1 <= n {
            if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] as i32 * base_pow[i - 1] as i32 % MOD {
                vo += 1;
                i += k;
            } else {
                i += 1;
            }
        }
    }
    return vo;
}

fn main() {
    let mut rng = StdRng::seed_from_u64(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    let mut s = String::new();

    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: i32 = t.trim().parse().unwrap();

    while t > 0 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        let n: usize = parts[0].parse().unwrap();
        let l: i32 = parts[1].parse().unwrap();
        let r: i32 = parts[2].parse().unwrap();
        
        s = parts[3].to_string();

        let mut base_pow: Vec<i32> = vec![0; N+1];
        let mut hash: Vec<i32> = vec![0; N];

        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp *= BASE;
            base_pow[i] = (bp * BASE) % MOD;
            hash[i] = (bp * s.chars().nth(i-1).unwrap() as i32) % MOD;
            hash[i] = (hash[i] + hash[i-1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if let val = check(mid, n, s.chars().collect(), base_pow.clone(), hash.clone()) >= l {
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
                    if let val = check(mid, n, s.chars().collect(), base_pow.clone(), hash.clone()) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }
        
        println!("");
        t -= 1;
    }
}