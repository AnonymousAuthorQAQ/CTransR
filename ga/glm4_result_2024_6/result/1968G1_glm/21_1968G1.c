use rand::Rng;
use std::io::{self, Read};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    rand::random::<u64>() * rand::random::<u64>()
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r { return l; }
    rand::random::<u64>() % (r - l) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 { return 1; }
    let r = power(a, b / 2);
    if b & 1 == 1 { r * r % MOD * a % MOD } else { r * r % MOD }
}

fn check(k: usize, hash: &[u64], base_pow: &[u64], n: usize) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k - 1] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut nums = lines.next().unwrap().split_whitespace();
        let n: usize = nums.next().unwrap().parse().unwrap();
        let l: usize = nums.next().unwrap().parse().unwrap();
        let r: usize = nums.next().unwrap().parse().unwrap();
        let s = lines.next().unwrap().chars().map(|c| c as u64).collect::<Vec<u64>>();

        let mut base = rand2(200, MOD - 1);
        let mut hash = vec![0; n + 1];
        let mut base_pow = vec![1; n + 1];

        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = hash[i - 1] * base % MOD + s[i];
            hash[i] %= MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow, n) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i > check(lower, &hash, &base_pow, n) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow, n) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            println!("{}", lower);
        }
    }
}