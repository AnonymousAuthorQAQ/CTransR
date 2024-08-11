use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let random_number = rand::random::<u64>() as u64 * 1u64 * rand::random::<u64>() as u64;
    random_number
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
    if b & 1 != 0 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, n: usize, hash: &Vec<u64>, base_pow: &Vec<u64>) -> usize {
    let mut vo = 0;
    for i in 1..=n {
        if i + k - 1 <= n {
            if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
                vo += 1;
                i += k;
            } else {
                i += 1;
            }
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = input[0];
        let l = input[1];
        let r = input[2];
        let s = lines.next().unwrap().unwrap();

        let mut hash: Vec<u64> = vec![0; N + 2];
        let mut base_pow: Vec<u64> = vec![0; N + 1];

        let mut bp: u64 = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * 1u64 * BASE % MOD;
            base_pow[i] = bp * 1u64 * BASE % MOD;
            hash[i] = bp * 1u64 * s.chars().nth(i).unwrap() as u64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: usize;

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