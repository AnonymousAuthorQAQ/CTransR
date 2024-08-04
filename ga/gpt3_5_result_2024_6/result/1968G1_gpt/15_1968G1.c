use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: u32 = 1000000007;

fn rand1() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>() * rng.gen::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: i32, b: i32) -> u32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 != 0 {
        return r as u64 * r as u64 % MOD as u64 * a as u64 % MOD as u64;
    }
    r as u64 * r as u64 % MOD as u64
}

fn check(k: usize, hash: &[u32]) -> i32 {
    let mut vo = 0;
    for i in 1..=hash.len() - k {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] as u64 * (base_pow[i - 1] as u64) % MOD as u64{
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        handle.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split(" ").collect();
        let n: usize = parts[0].parse().unwrap();
        let l: i32 = parts[1].parse().unwrap();
        let r: i32 = parts[2].parse().unwrap();
        let s: Vec<char> = parts[3].chars().collect();

        let mut hash = vec![0; N+1];
        let mut base_pow = vec![0; N+1];
        let mut bp = 1;
        
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp as u64 * BASE as u64 % MOD as u64;
            base_pow[i] = bp as u32;
            hash[i] = (bp as u64 * s[i-1] as u64) % MOD as u64;
            hash[i] = (hash[i] + hash[i - 1]) % MOD as u64;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, &hash)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        
        print!("{}",lower);

        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(mid, &hash)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    };
                }
            }
            print!("{}",lower);
        }

        println!();
    }
}