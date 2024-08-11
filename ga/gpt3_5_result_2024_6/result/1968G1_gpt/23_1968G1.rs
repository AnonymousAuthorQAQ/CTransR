use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    timestamp.wrapping_mul(rand::random::<u64>()).wrapping_mul(rand::random::<u64>())
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r.wrapping_mul(r).wrapping_mul(a) % MOD;
    }
    r.wrapping_mul(r) % MOD
}

fn check(hash: &Vec<u64>, base_pow: &Vec<u64>, n: usize, k: usize) -> u64 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1].wrapping_add(MOD).wrapping_sub(hash[i - 1])) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let mut base = rand2(200, MOD - 1);
    
    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s: &str = iter.next().unwrap();
        
        let mut base_pow: Vec<u64> = vec![1; n + 1];
        let mut hash: Vec<u64> = vec![0; n + 1];
        
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1].wrapping_mul(base) % MOD;
            hash[i] = (hash[i - 1].wrapping_mul(base) + s.chars().nth(i - 1).unwrap() as u64) % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;
        
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, n, mid)) >= l {
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