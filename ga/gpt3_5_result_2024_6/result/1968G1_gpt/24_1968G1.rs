use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let rnd: u64 = rand::random();
    rnd.wrapping_mul(1u64.wrapping_mul(rnd))
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

fn check(hash: &Vec<u64>, n: usize, k: usize) -> u64 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1].wrapping_add(MOD).wrapping_sub(hash[i - 1])) % MOD == hash[k] \
            .wrapping_mul(power(base_pow[i - 1], MOD)) % MOD {
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
    let t: usize = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let input: Vec<usize> = stdin.lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        let n = input[0];
        let l = input[1];
        let r = input[2];
        let s = input[3].clone();

        let mut hash: Vec<u64> = vec![0; n + 1];
        let mut base_pow: Vec<u64> = vec![1; n + 1];
        let mut bp = 1;
        
        for i in 1..=n {
            bp = bp.wrapping_mul(BASE) % MOD;
            base_pow[i] = bp.wrapping_mul(BASE) % MOD;
            hash[i] = bp.wrapping_mul(s.chars().nth(i - 1).unwrap() as u64) % MOD;
            hash[i] = (hash[i].wrapping_add(hash[i - 1])) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, n, mid)) >= l {
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
                    if (yy = check(&hash, n, mid)) >= i {
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