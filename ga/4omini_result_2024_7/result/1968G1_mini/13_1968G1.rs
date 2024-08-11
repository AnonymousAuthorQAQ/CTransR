use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1000000007;

fn power(a: usize, mut b: usize) -> usize {
    let mut result = 1;
    let mut a = a;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % MOD;
        }
        a = a * a % MOD;
        b /= 2;
    }
    result
}

fn check(n: usize, k: usize, s: &[usize], hash: &[usize], base_pow: &[usize]) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1]) % MOD {
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
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let s: String = lines.next().unwrap().unwrap();

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let base: usize = rng.gen_range(200..MOD as usize);

        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] * base + s.as_bytes()[i - 1] as usize) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = check(n, mid, &s.as_bytes().iter().map(|&b| b as usize).collect::<Vec<_>>(), &hash, &base_pow);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        let mut output = vec![lower.to_string()];

        let mut yy = check(n, lower, &s.as_bytes().iter().map(|&b| b as usize).collect::<Vec<_>>(), &hash, &base_pow);
        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(n, mid, &s.as_bytes().iter().map(|&b| b as usize).collect::<Vec<_>>(), &hash, &base_pow);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            output.push(lower.to_string());
        }

        println!("{}", output.join(" "));
    }
}