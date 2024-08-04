use std::io::{self, BufRead};

fn rand1() -> u32 {
    rand::random::<u32>() * rand::random::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

const N: usize = 200005;
const MOD: u32 = 1000000007;

fn power(a: u32, b: u32) -> u32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 != 0 {
        return r.wrapping_mul(r).wrapping_mul(a) % MOD;
    }
    return r.wrapping_mul(r) % MOD;
}

fn check(k: usize, n: usize, s: &str, hash: &mut [u32; N], base_pow: &mut [u32; N+1]) -> u32 {
    let mut vo = 0;
    for i in 1..=n {
        if i + k - 1 <= n {
            if (hash[i + k - 1].wrapping_add(MOD).wrapping_sub(hash[i - 1])) % MOD ==
                hash[k] as u64 * base_pow[i - 1] as u64 % MOD as u64 {
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
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let tokens: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = tokens[0];
        let l = tokens[1];
        let r = tokens[2];
        let s = tokens[3].to_string();

        let mut base = rand2(200, MOD - 1);
        let mut base_pow = vec![1u32; N+1];
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1].wrapping_mul(base) % MOD;
        }

        let mut hash = vec![0u32; N];
        for i in 1..=n {
            hash[i] = base_pow[i - 1].wrapping_mul(s.as_bytes()[i - 1] as u32) % MOD;
            hash[i] = (hash[i].wrapping_add(hash[i - 1])) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: u32;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &s, &mut hash, &mut base_pow)) >= l as u32 {
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
                    if (yy = check(mid, n, &s, &mut hash, &mut base_pow)) >= i as u32 {
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