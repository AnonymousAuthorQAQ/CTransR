use rand::Rng;
use std::time::SystemTime;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    rand::random::<u64>() * rand::random::<u64>()
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    rand::random::<u64>() % (r - l + 1) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, hash: &[u64], base_pow: &[u64]) -> usize {
    let mut vo = 0;
    for i in 1..=N {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let now = SystemTime::now();
    let seed: u64 = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64;
    rng.seed(&[seed]);

    let mut t = 0;
    loop {
        println!("Enter the number of test cases: ");
        t = read_int();

        for _ in 0..t {
            let mut n = 0;
            let mut l = 0;
            let mut r = 0;
            println!("Enter the length of the string: ");
            n = read_int();
            println!("Enter the range [l, r]: ");
            l = read_int();
            r = read_int();
            println!("Enter the string: ");
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let s = s.trim();

            let mut base_pow = vec![0; N + 1];
            let mut hash = vec![0; N + 1];
            let mut bp = 1;
            base_pow[0] = 1;
            for (i, c) in s.chars().enumerate() {
                let val = c as u64;
                bp = bp * val % MOD;
                base_pow[i + 1] = bp;
                hash[i + 1] = bp;
                if i > 0 {
                    hash[i + 1] = (hash[i + 1] + hash[i]) % MOD;
                }
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
}

fn read_int() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}