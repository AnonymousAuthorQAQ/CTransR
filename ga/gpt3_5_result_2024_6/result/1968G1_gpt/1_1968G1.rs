use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    rand::random::<u64>() * 1u64 * rand::random::<u64>()
}

fn rand2(l: usize, r: usize) -> usize {
    if l == r {
        return l;
    }
    rand1() as usize % (r - l) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(k: usize, n: usize, hash: &Vec<u64>, base_pow: &Vec<u64>) -> usize {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let l: usize = parts[1].parse().unwrap();
        let r: usize = parts[2].parse().unwrap();
        let s: &str = parts[3];

        let mut hash = vec![0; N + 1];
        let mut base_pow = vec![0; N + 1];
        let mut bp = 1;

        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * BASE % MOD;
            base_pow[i] = bp * BASE % MOD;
            hash[i] = bp * s.as_bytes()[i - 1] as u64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &hash, &base_pow)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

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
            print!("{}", lower);
        }

        println!();
    }
}