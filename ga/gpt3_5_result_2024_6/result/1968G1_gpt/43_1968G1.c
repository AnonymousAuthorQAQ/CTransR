use std::time::SystemTime;
use std::collections::HashMap;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    (current_time.as_secs() as u32 * current_time.subsec_nanos())
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
    if b & 1 == 1 {
        r * r % MOD * a % MOD
    } else {
        r * r % MOD
    }
}

fn check(s: &str, k: usize, base_pow: &Vec<i64>) -> usize {
    let mut vo = 0;
    let mut hash = HashMap::new();
    
    hash.insert(0, 0);

    for i in 1..=s.len() {
        let val = (hash[&(i - 1)] + s.chars().nth(i - 1).unwrap() as i64) % MOD;
        hash.insert(i, val);
    }

    for i in 1..=s.len() - k + 1 {
        let substr_hash = (hash[&i + k - 1] + MOD - hash[&i - 1]) % MOD;
        let base_val = hash[&k] * base_pow[i - 1] % MOD;

        if substr_hash == base_val {
            vo += 1;
        }
    }

    vo
}

fn main() {
    let mut base_pow = vec![1; N + 1];
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: isize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut inputs = input.split_whitespace();

        let n: usize = inputs.next().unwrap().parse().unwrap();
        let l: usize = inputs.next().unwrap().parse().unwrap();
        let r: usize = inputs.next().unwrap().parse().unwrap();
        let s: String = inputs.next().unwrap().parse().unwrap();

        let base = rand2(200, MOD - 1);
        base_pow.iter().enumerate().for_each(|(i, &bp)| {
            base_pow[i] = bp * base % MOD;
        });

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(&s, mid, &base_pow);

            if yy >= l {
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
                    yy = check(&s, mid, &base_pow);

                    if yy >= i {
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