use std::time::SystemTime;

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u32 {
    rand::random::<u32>() * 1u32 * rand::random::<u32>()
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
    let r = power(a, b/2);
    if b%2 == 1 {
        return (r * r % MOD * a % MOD) % MOD;
    }
    (r * r) % MOD
}

fn check(hash: &Vec<u64>, base_pow: &Vec<u64>, k: usize) -> u32 {
    let mut vo: u32 = 0;
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
    let mut rng = rand::thread_rng();
    let mut base_pow: Vec<u64> = vec![1; N + 1];
    let mut hash: Vec<u64> = vec![0; N];
    let mut s = String::new();
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32;

    let mut t = 0;
    let n = 0;
    let l = 0;
    let r = 0;
  
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split(" ");
    t = parts.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split(" ").collect();

        n = parts[0].trim().parse().unwrap();
        l = parts[1].trim().parse().unwrap();
        r = parts[2].trim().parse().unwrap();
        s = parts[3].to_string();

        let base = rand2(200, MOD - 1);

        let mut bp = 1;
        for i in 1..=n {
            bp = bp * BASE % MOD;
            base_pow[i] = bp * BASE % MOD;
            hash[i] = (bp * s[i-1] as u64) % MOD;
            hash[i] = (hash[i] + hash[i-1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy: u32;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, mid)) >= l {
                lower = mid;
            }else {
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
                    if (yy = check(&hash, &base_pow, mid)) >= i {
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