use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = rand::thread_rng();
    rng.gen::<u32>() * 1u32 * (rng.gen::<u32>() + timestamp) as u32
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return (r * r % MOD) * a % MOD;
    }
    (r * r) % MOD
}

fn check(k: usize, n: usize, base_pow: &Vec<i64>, hash: &Vec<i64>) -> i32 {
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
    vo
}

fn main() {
    let mut base = 0;
    let mut base_pow = vec![1; N + 1];

    let t: i32;
    std::io::stdin().read_line(&mut t);

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        let mut iter = input.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: i32 = iter.next().unwrap().parse().unwrap();
        let r: i32 = iter.next().unwrap().parse().unwrap();
        let s: &str = iter.next().unwrap();

        let mut hash = vec![0; N];

        let mut bp = 1;
        for i in 1..=n {
            bp = bp * BASE % MOD;
            base_pow[i] = bp * BASE % MOD;
            hash[i] = bp * s.as_bytes()[i - 1] as i64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(mid, n, &base_pow, &hash);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        println!("{}", lower);

        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(mid, n, &base_pow, &hash);
                    if yy >= i {
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