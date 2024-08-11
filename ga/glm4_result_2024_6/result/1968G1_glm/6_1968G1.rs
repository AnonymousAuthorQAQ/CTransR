use std::io::{self, BufRead};
use rand::{Rng, thread_rng};
use std::time::SystemTime;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let mut rng = thread_rng();
    rng.gen()
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: u64, b: u64, modu: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2, modu);
    if b % 2 == 0 {
        return r * r % modu;
    }
    return r * r % modu * a % modu;
}

fn check(k: usize, hash: &[u64], base_pow: &[u64]) -> usize {
    let mut vo = 0;
    for i in 1..=N {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        let n: usize;
        let l: usize;
        let r: usize;

        {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            n = parts[0];
            l = parts[1];
            r = parts[2];
            s = parts[3];
        }

        let mut hash = vec![0; N + 2];
        let mut base_pow = vec![0; N + 1];

        let bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * rand2(200, MOD) % MOD;
            hash[i] = base_pow[i] * s.as_bytes()[i - 1] as u64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i > check(lower, &hash, &base_pow) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow) >= i {
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