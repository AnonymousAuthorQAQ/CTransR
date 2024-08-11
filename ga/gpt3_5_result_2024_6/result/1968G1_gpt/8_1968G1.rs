use std::time::SystemTime;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let seed = current_time.as_secs() as u64;
    let rand_val = seed.wrapping_mul(seed).wrapping_mul(seed);
    rand_val
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

fn check(k: usize, n: usize, hash: &[u64], base_pow: &[u64]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let mut base_pow: Vec<u64> = vec![1; N + 1];

    let t: usize = 1; // Set the number of test cases
    let n: usize = 10; // Set the length of the string
    let l: usize = 2; // Set the lower limit
    let r: usize = 4; // Set the upper limit
    let s: String = String::from("abacabadac");

    let mut hash: Vec<u64> = vec![0; N];
    let mut bp: u64 = 1;

    for i in 1..=n {
        bp = bp.wrapping_mul(200).wrapping_rem(MOD);
        base_pow[i] = bp.wrapping_mul(200).wrapping_rem(MOD);
        hash[i] = bp.wrapping_mul(s.chars().nth(i - 1).unwrap() as u64).wrapping_rem(MOD);
        hash[i] = (hash[i] + hash[i - 1]) % MOD;
    }

    let mut lower: usize = 0;
    let mut upper: usize = n + 1;
    let mut yy: usize;

    while upper - lower > 1 {
        let mid = lower + (upper - lower) / 2;
        if (yy = check(mid, n, &hash, &base_pow)) >= l {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    println!("{}", lower);

    for i in l + 1..=r {
        if i > yy {
            upper = lower + 1;
            lower = 0;
            while upper - lower > 1 {
                let mid = lower + (upper - lower) / 2;
                if (yy = check(mid, n, &hash, &base_pow)) >= i {
                    lower = mid;
                } else {
                    upper = mid;
                }
            }
        }
        println!("{}", lower);
    }
}