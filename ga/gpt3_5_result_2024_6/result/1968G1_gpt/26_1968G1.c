use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    now as u64 * (now as u64).wrapping_mul(now as u64)
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() as u32 % (r - l) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 == 1 {
        return (r * r % MOD * a % MOD) % MOD;
    }
    (r * r) % MOD
}

fn check(hash: &Vec<u32>, base_pow: &Vec<u32>, k: usize, n: usize) -> usize {
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
    let mut rng = rand::thread_rng();
    let mut base = rand2(200, MOD as u32 - 1);
    let mut base_pow: Vec<u32> = vec![1; N + 1];
    let mut hash: Vec<u32> = Vec::with_capacity(N + 1);

    let mut buf1 = 1u32;
    for i in 1..=N {
        buf1 = (buf1 as u64 * base as u64 % MOD as u64) as u32;
        base_pow[i] = buf1 as u32;
        hash.push((buf1 as u64 * i as u64 % MOD as u64) as u32);
        hash[i] = (hash[i] + hash[i - 1]) % MOD as u32;
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        let n: usize = parts[0].parse().unwrap();
        let l: usize = parts[1].parse().unwrap();
        let r: usize = parts[2].parse().unwrap();
        let s = parts[3].as_bytes();

        let mut yy = 0;
        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, mid, n)) >= l {
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
                    if (yy = check(&hash, &base_pow, mid, n)) >= i {
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