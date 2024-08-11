use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>() * 1u32 * rng.gen::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
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
    if b & 1 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(hash: &Vec<u64>, base_pow: &Vec<u64>, n: usize, k: usize) -> u32 {
    let mut vo = 0;
    for i in 1..=n-k {
        if (hash[i+k-1] + MOD - hash[i-1]) % MOD == hash[k] * base_pow[i-1] % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut base = rand2(200, MOD-1);

    let t: u32;
    scanf("%d",&t);
    while t > 0 {
        let n: usize;
        let l: u32;
        let r: u32;
        let s: String = String::with_capacity(N+2);

        scanf("%d%d%d%s",&n,&l,&r,&s);

        let mut base_pow: Vec<u64> = Vec::with_capacity(n+1);
        let mut bp: u64 = 1;
        base_pow.push(bp);
        for i in 1..=n {
            bp = bp * base % MOD;
            base_pow.push(bp);
        }

        let mut hash: Vec<u64> = Vec::with_capacity(n+1);
        hash.push(0);
        for i in 1..=n {
            hash.push(bp * s.chars().nth(i-1) as u64 % MOD);
            hash[i] = (hash[i] + hash[i-1]) % MOD;
        }

        let mut lower: u32 = 0;
        let mut upper: usize = n+1;
        let mut yy: u32;

        while upper-lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, n, mid)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{}", lower);

        for i in l+1..=r {
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

        println!("");
        t -= 1;
    }
}