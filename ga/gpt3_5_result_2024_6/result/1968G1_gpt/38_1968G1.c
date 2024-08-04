use std::io;
use rand::Rng;

fn rand1() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>() * rng.gen::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

const N: usize = 200005;
const MOD: i32 = 1000000007;

fn power(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 != 0 {
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(hash: &[i32; N], base_pow: &[i32; N+1], n: usize, k: usize) -> i32 {
    let mut vo = 0;
    for i in 1..=n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i-1] % MOD {
            vo += 1;
            let i = i + k;
        } else {
            let i = i + 1;
        }
    }
    vo
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: i32 = iter.next().unwrap().parse().unwrap();
        let r: i32 = iter.next().unwrap().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s: &str = input.trim();

        let mut base: i32 = rand2(200, MOD as u32 - 1);
        let mut base_pow: [i32; N+1] = [0; N+1];
        let mut hash: [i32; N] = [0; N];

        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp.wrapping_mul(base) % MOD;
            base_pow[i] = bp.wrapping_mul(base) % MOD;
            hash[i] = bp.wrapping_mul(s.as_bytes()[i - 1] as i32) % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: i32;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, n, mid)) >= l {
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
                    if (yy = check(&hash, &base_pow, n, mid)) >= i {
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