use std::io;
use rand::Rng;

const N: usize = 200005;
const MOD: usize = 1_000_000_007;

fn main() {
    let mut rng = rand::thread_rng();
    let base: usize = rng.gen_range(200..MOD);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let stdin = io::stdin();
    let mut input_string = String::new();

    stdin.read_line(&mut input_string).unwrap();
    let t: usize = input_string.trim().parse().unwrap();
    
    for _ in 0..t {
        input_string.clear();
        stdin.read_line(&mut input_string).unwrap();
        let mut parts = input_string.split_whitespace();
        
        let n: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let mut s: Vec<char> = parts.next().unwrap().chars().collect();
        s.insert(0, ' '); // To make it 1-indexed

        let mut base_pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        let mut bp = 1;
        for i in 1..=n {
            bp = (bp * base) % MOD;
            base_pow[i] = bp;
            hash[i] = (hash[i - 1] + (s[i] as usize) * base_pow[i]) % MOD) % MOD;
        }

        let (mut lower, mut upper) = (0, n + 1);
        
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = check(mid, n, &hash, &base_pow, &s);
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
                    let yy = check(mid, n, &hash, &base_pow, &s);
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

fn check(k: usize, n: usize, hash: &Vec<usize>, base_pow: &Vec<usize>, s: &Vec<char>) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1]) % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}