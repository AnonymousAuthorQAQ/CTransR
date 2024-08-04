use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i32 = 1000000007;

fn rand1() -> u32 {
    let val = rand::random::<u32>();
    val.wrapping_mul(val)
}

fn rand2(l: usize, r: usize) -> usize {
    if l == r {
        return l;
    }
    rand1() as usize % (r - l) + l
}

fn power(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r.wrapping_mul(r).wrapping_mul(a) % MOD;
    }
    return r.wrapping_mul(r) % MOD;
}

fn check(k: usize, n: usize, hash: &Vec<i32>, base_pow: &Vec<i32>) -> i32 {
    let mut vo = 0;
    for i in 1..=n {
        if i + k - 1 <= n {
            if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
                vo += 1;
            }
        }
    }
    vo
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();
    let t: i32 = buf.trim().parse().unwrap();

    for _ in 0..t {
        buf.clear();
        input.read_line(&mut buf).unwrap();
        let values: Vec<i32> = buf.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let n = values[0] as usize;
        let l = values[1] as usize;
        let r = values[2] as usize;
        let s = values[3..].join("").chars().collect::<Vec<char>>();

        let mut hash = vec![0; N + 1];
        let mut base_pow = vec![0; N + 1];

        let mut bp = 1;
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp.wrapping_mul(200) % MOD;
            base_pow[i] = bp.wrapping_mul(200) % MOD;
            hash[i] = (bp.wrapping_mul(s[i - 1] as i32) % MOD + hash[i - 1]) % MOD;
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
        print!("{} ", lower);

        for i in l+1..=r {
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
            print!("{} ", lower);
        }

        println!();
    }
}