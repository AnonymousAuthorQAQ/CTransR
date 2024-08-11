use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seed = since_the_epoch.as_micros();
    let rand_val = seed.wrapping_mul(seed);
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
    if b & 1 == 1 {
        return r.wrapping_mul(r).wrapping_mul(a) % MOD;
    }
    return r.wrapping_mul(r) % MOD;
}

fn check(hash: &Vec<u64>, base_pow: &Vec<u64>, k: usize) -> u64 {
    let mut vo = 0;
    let n = hash.len();
    for i in 1..=n-k {
        if (hash[i+k-1].wrapping_sub(hash[i-1]).wrapping_rem(MOD) == hash[k].wrapping_mul(base_pow[i-1]).wrapping_rem(MOD)) {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let t: usize = s.trim().parse().unwrap();

    for _i in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s: &str = iter.next().unwrap();

        let mut hash = vec![0; N+2];
        let mut base_pow = vec![1; N+1];

        let mut bp = 1;
        for i in 1..=n {
            bp = bp.wrapping_mul(BASE).wrapping_rem(MOD);
            base_pow[i] = bp.wrapping_mul(BASE).wrapping_rem(MOD);
            hash[i] = bp.wrapping_mul(s.as_bytes()[i-1] as u64).wrapping_rem(MOD);
            hash[i] = hash[i].wrapping_add(hash[i-1]).wrapping_rem(MOD);
        }

        let (mut lower, mut upper) = (0, n+1);
        let mut yy;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, mid)) >= l as u64 {
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
                    if (yy = check(&hash, &base_pow, mid)) >= i as u64 {
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