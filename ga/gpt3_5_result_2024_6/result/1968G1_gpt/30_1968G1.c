use std::time::SystemTime;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u32 {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let seed = current_time.as_secs() as u64;
    let hashed_seed = seed.wrapping_mul(seed);
    
    hashed_seed as u32
}

fn rand2(l: u32, r: u32) -> u32 {
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
    if b & 1 == 1 {
        r.wrapping_mul(r).wrapping_mul(a) % MOD
    } else {
        r.wrapping_mul(r) % MOD
    }
}

fn check(k: usize, n: usize, hash: &Vec<u64>, base_pow: &Vec<u64>) -> u64 {
    let mut vo = 0;
    let mut i = 1;
    
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i-1] % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    
   vo
}

fn main() {
    let t: u32;
    
    let mut current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let seed = current_time.as_secs() as u64;
    let BASE = rand2(200, MOD - 1);
    
    let mut base_pow: Vec<u64> = vec![1; N + 1];
    
}
