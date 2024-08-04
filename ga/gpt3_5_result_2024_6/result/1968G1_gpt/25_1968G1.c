use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: usize = 1000000007;

fn rand1() -> usize { rand() * 1u32 * rand() as usize }
fn rand2(l: usize, r: usize) -> usize { if l == r { return l; } rand1() % (r - l) + l }

fn rand() -> usize {
    let nsecs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u32;
    nsecs as usize
}

fn power(a: usize, b: usize) -> usize {
    if b == 0 { return 1; }
    let r = power(a, b / 2);
    if b % 2 == 1 { return r * r % MOD * a % MOD; }
    return r * r % MOD;
}

fn check(hash: &Vec<usize>,  base_pow: &Vec<usize>, k: usize, n: usize) -> usize {
    let mut vo = 0;
    for i in 1..=n {
        if i + k - 1 <= n {
            if (hash[i+k-1] + MOD - hash[i-1]) % MOD == hash[k] * base_pow[i-1] % MOD {
                vo += 1;
            }
        }
    }
    vo
}

fn main() {
    let mut base = rand2(200, MOD - 1);
    
    let t: usize;
    let mut input: String = String::new();
    
    std::io::stdin().read_line(&mut input).unwrap();
    t = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let l: usize = parts[1].parse().unwrap();
        let r: usize = parts[2].parse().unwrap();
        let s: &str = parts[3];
        
        let mut hash: Vec<usize> = vec![0; N+1];
        let mut base_pow: Vec<usize> = vec![0; N+1];
        
        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow[i] = bp * BASE % MOD;
            hash[i] = bp * s.chars().nth(i-1).unwrap() as usize % MOD;
            hash[i] = hash[i] + hash[i-1] % MOD;
        }
        
        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;
        
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let vo = check(&hash, &base_pow, mid, n);
            
            if vo >= l { lower = mid; } else { upper = mid; }
        }
        
        print!("{}", lower);
        
        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(&hash, &base_pow, mid, n);
                    
                    if yy >= i { lower = mid; } else { upper = mid; }
                }
            }
            
            print!("{}", lower);
        }
    }
}