use std::time::SystemTime;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    (current_time % 1000) * (current_time % 2000)
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, n: usize, hash: &Vec<u64>, base_pow: &Vec<u64>) -> u64 {
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
    let mut s: Vec<char> = vec![' '; N + 2];
    let mut hash: Vec<u64> = vec![0; N];
    let mut base_pow: Vec<u64> = vec![0; N + 1];

    let t: usize = 1;
    let n: usize = 10;
    let l: usize = 2;
    let r: usize = 4;
    let input_string = "abracadabra";

    for (i, c) in input_string.chars().enumerate() {
        s[i + 1] = c;
    }

    let mut BASE: u64 = rand2(200, MOD - 1);

    let mut bp: u64 = 1;
    base_pow[0] = 1;
    for i in 1..=n {
        bp = (bp * BASE) % MOD;
        base_pow[i] = (bp * BASE) % MOD;
        hash[i] = (bp * s[i] as u64) % MOD;
        hash[i] = (hash[i] + hash[i - 1]) % MOD;
    }

    let mut lower = 0;
    let mut upper = n + 1;
    let mut yy: u64 = 0;

    while upper - lower > 1 {
        let mid = lower + (upper - lower) / 2;
        if (yy = check(mid, n, &hash, &base_pow)) >= l as u64 {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    print!("{}", lower);

    for i in (l + 1)..=r {
        if i > yy {
            upper = lower + 1;
            lower = 0;

            while upper - lower > 1 {
                let mid = lower + (upper - lower) / 2;
                if (yy = check(mid, n, &hash, &base_pow)) >= i as u64 {
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