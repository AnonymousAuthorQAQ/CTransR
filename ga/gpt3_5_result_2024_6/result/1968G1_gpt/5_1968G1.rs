use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = now.as_millis() as u32;
    return seed.wrapping_mul(seed);
}

fn rand2(l: usize, r: usize) -> usize {
    if l == r {
        return l;
    }
    return rand1() as usize % (r - l) + l;
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 == 1 {
        return r.wrapping_mul(r).wrapping_rem(MOD).wrapping_mul(a).wrapping_rem(MOD);
    }
    return r.wrapping_mul(r).wrapping_rem(MOD);
}

fn check(k: usize, n: usize, s: &Vec<char>, base_pow: &Vec<i64>, hash: &Vec<i64>) -> i32 {
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
    return vo;
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let t: usize = s.trim().parse().unwrap();

    for _ in 0..t {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let inputs: Vec<usize> = s.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let n = inputs[0];
        let l = inputs[1];
        let r = inputs[2];

        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let s: Vec<char> = s.trim().chars().collect();

        let mut hash = vec![0; N];
        let mut base_pow = vec![1; N + 1];
        let mut bp = 1;

        for i in 1..=n {
            bp = bp.wrapping_mul(rand2(200, MOD as usize - 1) as i64).wrapping_rem(MOD);
            base_pow[i] = bp.wrapping_mul(rand2(200, MOD as usize - 1) as i64).wrapping_rem(MOD);
            hash[i] = bp.wrapping_mul(s[i - 1] as i64).wrapping_rem(MOD);
            hash[i] = (hash[i] + hash[i - 1]).wrapping_rem(MOD);
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: i32;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &s, &base_pow, &hash)) >= l as i32 {
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
                    if (yy = check(mid, n, &s, &base_pow, &hash)) >= i {
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