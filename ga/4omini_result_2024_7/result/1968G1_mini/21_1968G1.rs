use rand::Rng;
use std::io;

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn power(a: usize, mut b: usize) -> usize {
    let mut r = 1;
    let mut a = a;
    while b > 0 {
        if b % 2 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        b /= 2;
    }
    r
}

fn check(hash: &[usize], base_pow: &[usize], k: usize, n: usize, l: usize) -> usize {
    let mut vo = 0;
    let target_hash = hash[k] * base_pow[0] % MOD;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == target_hash {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn rand2(l: usize, r: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(l..=r)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Not a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let tokens: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = tokens[0];
        let l = tokens[1];
        let r = tokens[2];
        let s = format!("{}{}", " ", 
            io::stdin().read_line(&mut String::new()).expect("Failed to read line"));

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        let base = rand2(200, MOD - 1);
        
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] + s[i..i+1].as_bytes()[0] as usize * base_pow[i]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(&hash, &base_pow, mid, n, l);
            if yy >= l {
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
                    yy = check(&hash, &base_pow, mid, n, l);
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