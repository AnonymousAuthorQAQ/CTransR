use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    rand::random::<u32>() * 1u32 * rand::random::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: i64, b: i32) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, hash: &Vec<i64>, base_pow: &Vec<i64>) -> i32 {
    let mut vo = 0;
    for i in 1..=hash.len() - k {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split(" ").collect();
        let n: i32 = parts[0].parse().unwrap();
        let l: i32 = parts[1].parse().unwrap();
        let r: i32 = parts[2].parse().unwrap();
        let s = parts[3];
        
        let mut hash = vec![0; N];
        let mut base_pow = vec![0; N + 1];
        
        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * 1 * rand2(200, MOD - 1) % MOD;
            base_pow[i as usize] = bp * 1 * rand2(200, MOD - 1) % MOD;
            hash[i as usize] = bp * 1 * s.chars().nth(i as usize).unwrap() as i64 % MOD;
            hash[i as usize] = (hash[i as usize] + hash[i as usize - 1] as i64) % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;
        
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid as usize, &hash, &base_pow)) >= l as usize {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);
        
        for i in (l + 1)..=r {
            if i > yy as i32 {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(mid as usize, &hash, &base_pow)) >= i as usize {
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