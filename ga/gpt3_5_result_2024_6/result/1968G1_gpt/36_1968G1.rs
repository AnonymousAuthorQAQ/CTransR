use std::{env, usize};
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u32 {
    let random_num = rand::random::<u32>();
    random_num.wrapping_mul(1).wrapping_mul(random_num)
}

fn rand2(l: u32, r:u32) -> u32 {
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
        return r.wrapping_mul(r).wrapping_rem(MOD).wrapping_mul(a).wrapping_rem(MOD);
    }
    r.wrapping_mul(r).wrapping_rem(MOD)
}

fn check(k: usize, hash: &Vec<u64>, base_pow: &Vec<u64>) -> usize {
    let mut vo = 0;
    let n = hash.len();
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] %MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Command line arguments
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input_line = String::new();

    handle.read_line(&mut input_line).unwrap();
    let mut iter = input_line.trim().split(' ');

    let n: usize = iter.next().unwrap().parse().expect("Expected a number");
    let l: usize = iter.next().unwrap().parse().expect("Expected a number");
    let r: usize = iter.next().unwrap().parse().expect("Expected a number");
    input_line.clear();
    
    handle.read_line(&mut input_line).unwrap();
    let s: Vec<char> = input_line.trim().chars().collect();

    let mut hash: Vec<u64> = vec![0; N + 2];
    let mut base_pow: Vec<u64> = vec![0; N + 1];

    let mut bp = 1;
    base_pow[0] = 1;
    for i in 1..=n {
        bp = bp.wrapping_mul(200) % MOD;
        base_pow[i] = bp.wrapping_mul(200) % MOD;
        hash[i] = bp.wrapping_mul(s[i] as u64) % MOD;
        hash[i] = (hash[i] + hash[i - 1]) % MOD;
    }

    let mut lower = 0;
    let mut upper = n + 1;
    let mut yy;

    while upper - lower > 1 {
        let mid = lower + (upper - lower) / 2;
        if (yy = check(mid, &hash, &base_pow)) >= l {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    print!("{}", lower);

    for i in l + 1..=r {
        if i > yy {
            let upper = lower + 1;
            let mut lower = 0;
            while upper - lower > 1 {
                let mid = lower + (upper - lower) / 2;
                if (yy = check(mid, &hash, &base_pow)) >= i {
                    lower = mid;
                } else {
                    upper = mid;
                }
            }
        }
        print!("{}", lower);
    }
    print!("\n");
}