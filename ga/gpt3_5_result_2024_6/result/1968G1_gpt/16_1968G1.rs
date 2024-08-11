use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut rng = now.as_secs() as u64;
    rng = (rng * 1103515245 + 12345) & 0x7fffffff;
    (rng >> 16) & 0x7fff
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
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, hash: &Vec<u64>, base_pow: &Vec<u64>) -> u32 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= hash.len() {
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
    let mut base_pow = vec![0; N + 1];
    base_pow[0] = 1;
    for i in 1..=N {
        base_pow[i] = base_pow[i - 1] * BASE % MOD;
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: u32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let values: Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let n = values[0] as usize;
        let l = values[1] as u32;
        let r = values[2] as u32;
        let s = values[3].trim().to_string();

        let mut hash = vec![0; N];
        let mut temp_hash: u64 = 0;
        for i in 0..n {
            temp_hash = temp_hash * BASE % MOD;
            temp_hash = (temp_hash + (s.chars().nth(i).unwrap() as u64)) % MOD;
            hash[i] = temp_hash;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, &hash, &base_pow)) >= l {
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
                    if (yy = check(mid, &hash, &base_pow)) >= i {
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