use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn rand1() -> usize {
    let mut rng = rand::thread_rng();
    let a: usize = rng.gen();
    let b: usize = rng.gen();
    a.wrapping_mul(b)
}

fn rand2(l: usize, r: usize) -> usize {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let mut r = power(a, b / 2);
    r = r.wrapping_mul(r) % MOD;
    if b % 2 == 1 {
        r = r.wrapping_mul(a) % MOD;
    }
    r
}

fn check(s: &[usize], hash: &[usize], base_pow: &[usize], n: usize, k: usize) -> usize {
    let mut vo = 0;
    for i in 1..=n {
        if i + k - 1 <= n {
            if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
                vo += 1;
                i += k - 1; // skip k characters
            } else {
                i += 1;
            }
        } else {
            break;
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: Vec<usize> = parts.next().unwrap().chars().map(|c| c as usize).collect();

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let mut bp = 1;

        let base = rand2(200, MOD - 1); // Random BASE

        for i in 1..=n {
            bp = bp.wrapping_mul(base) % MOD;
            base_pow[i] = bp;
            hash[i] = (hash[i - 1].wrapping_mul(base) + s[i - 1]) % MOD; // s[i - 1] as char is indexed from 0 in Rust
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&s, &hash, &base_pow, n, mid)) >= l {
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
                    if (yy = check(&s, &hash, &base_pow, n, mid)) >= i {
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