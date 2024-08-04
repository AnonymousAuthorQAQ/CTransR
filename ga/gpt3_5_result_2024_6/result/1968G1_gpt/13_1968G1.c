use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    rand::random::<u64>() * 1u64 * rand::random::<u64>()
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
    let r = power(a, b/2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(k: usize, n: usize, s: &str, hash: &mut Vec<u64>, base_pow: &Vec<u64>) -> u64 {
    let mut vo = 0;
    for i in 1..=n-k {
        if (hash[i+k-1] + MOD - hash[i-1]) % MOD == hash[k] * base_pow[i-1] % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    rand::srand(seed);

    let mut base: u64 = rand2(200, MOD-1);

    let mut base_pow: Vec<u64> = Vec::with_capacity(N+1);
    let mut bp: u64 = 1;
    base_pow.push(1);
    for i in 1..=N {
        bp = bp * base % MOD;
        base_pow.push(bp);
    }

    let mut t = 0;
    let mut inputs: Vec<(usize, usize, usize, String)> = Vec::new();
  
    t = 1;
  
    for _ in 0..t {
        let (n, l, r, s) = (200005, 200, MOD-1, "0".to_string());  // Input example
        inputs.push((n, l, r, s));
    }

    for input in inputs {
        let (n, l, r, s) = input;
        let mut hash:Vec<u64> = Vec::with_capacity(n+1);
        let mut hash_value: u64 = 0;
        for i in 1..=n {
            hash_value = base * s[i-1].to_digit(10).unwrap() as u64 % MOD;
            hash.push((hash_value + hash[i-1]) % MOD);
        }
        
        let mut lower = 0;
        let mut upper = n+1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
        
            if (yy = check(mid, n, &s, &mut hash, &base_pow)) >= l {
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
                    
                    if (yy = check(mid, n, &s, &mut hash, &base_pow)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }
        println!("");
    }
}