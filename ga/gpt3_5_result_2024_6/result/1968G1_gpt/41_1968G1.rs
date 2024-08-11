use rand::Rng;
use std::time::SystemTime;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>() * 1u64 * rng.gen::<u64>()
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
    if b & 1 == 1 {
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(hash: &Vec<u64>, base_pow: &Vec<u64>, k: usize) -> u64 {
    let mut vo = 0;
    let n = hash.len();
    let hash_k = hash[k];
    for i in 1..=n - k {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash_k * base_pow[i - 1] % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let base: u64 = rand2(200, MOD - 1);
    
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut split_input = input.split_whitespace();
        let n: usize = split_input.next().unwrap().parse().unwrap();
        let l: usize = split_input.next().unwrap().parse().unwrap();
        let r: usize = split_input.next().unwrap().parse().unwrap();
        let s: String = split_input.next().unwrap().parse().unwrap();

        let mut hash: Vec<u64> = Vec::with_capacity(n + 1);
        let mut base_pow: Vec<u64> = Vec::with_capacity(n + 1);
        let mut bp: u64 = 1;
        hash.push(bp * s[0] as u64 % MOD);
        base_pow.push(bp);

        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow.push(bp);
            hash.push((bp * s[i - 1] as u64 % MOD + hash[i - 1]) % MOD);
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: u64;
        
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, mid)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(&hash, &base_pow, mid)) >= i {
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