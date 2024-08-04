use std::time::{SystemTime, UNIX_EPOCH};
use std::char;
use std::cmp::Ordering;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    return (now as u32).wrapping_mul((now as u32).wrapping_mul(rand()) as u32);
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    } 
    return rand1() % (r - l) + l;
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b/2);
    if b & 1 == 1 {
        return (r * r % MOD * a) % MOD;
    }
    return (r * r) % MOD;
}

fn check(k: usize, n: usize, hash: &Vec<i64>, base_pow: &Vec<i64>) -> i32 {
    let mut vo = 0;
    let mut i = 1;
    
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == ((hash[k] * base_pow[i - 1]) % MOD) {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    
    return vo;
}

fn main() {
    let mut s: [char; N + 2] = ['0'; N + 2];
    let mut hash = vec![0; N];
    let mut base_pow = vec![0; N + 1];
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    while t > 0 {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        let n: usize = parts[0].parse().unwrap();
        let l: i32 = parts[1].parse().unwrap();
        let r: i32 = parts[2].parse().unwrap();
        
        let input_str = parts[3].as_bytes();
        for i in 0..n {
            s[i+1] = char::from(input_str[i]);
        }
        
        let mut bp = 1;
        base_pow[0] = 1;
        
        for i in 1..=n {
            bp = (bp * BASE) % MOD;
            base_pow[i] = (bp * BASE) % MOD;
            hash[i] = (bp * s[i] as i64) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;
        
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (check(mid, n, &hash, &base_pow) as i32) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        print!("{}", lower);
        
        for i in (l+1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(mid, n, &hash, &base_pow) as i32) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            
            print!("{}", lower);
        }
        
        println!();
        
        t -= 1;
    }
}