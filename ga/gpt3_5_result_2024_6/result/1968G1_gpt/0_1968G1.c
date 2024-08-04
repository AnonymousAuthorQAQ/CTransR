use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: u32 = 1000000007;

fn rand1() -> u32 {
    return rand::random::<u32>() * 1u32 * rand::random::<u32>();
}

fn rand2(l: usize, r: usize) -> usize {
    if l == r {
        return l;
    }
    return rand1() as usize % (r - l) + l;
}

fn power(a: i32, b: u32) -> i32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b/2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(k: usize, n: usize, s: String, hash: &Vec<u32>, base_pow: &Vec<u32>) -> i32 {
    let mut vo = 0;
    let bytes = s.as_bytes();
    for i in 1..=n {
        if i+k-1 <= n && (hash[i+k-1] + MOD - hash[i-1]) % MOD == hash[k] as u32 * base_pow[i-1] % MOD {
            vo += 1;
        }
    }
    return vo;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let t: i32 = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input_str = lines.next().unwrap();
        let inputs: Vec<&str> = input_str.split_whitespace().collect();
        let n: usize = inputs[0].parse().unwrap();
        let l: usize = inputs[1].parse().unwrap();
        let r: usize = inputs[2].parse().unwrap();
        let s: String = inputs[3].to_string();

        let mut hash = vec![0; N+1];
        let mut base_pow = vec![0; N+1];

        let mut bp: u32 = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * 1u32 * BASE % MOD;
            base_pow[i] = bp * 1u32 * BASE % MOD;
            hash[i] = bp * 1u32 * s.as_bytes()[i-1] as u32 % MOD;
            hash[i] = (hash[i] + hash[i-1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let c = check(mid, n, s.to_string(), &hash, &base_pow);
            if c >= l as i32 {
                lower = mid;
            } else {
                upper = mid;
            }
        }
    
        print!("{} ", lower);
    
        for i in l+1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let c = check(mid, n, s.to_string(), &hash, &base_pow);
                    if c >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{} ", lower);
        }
    
        println!();
    }
}