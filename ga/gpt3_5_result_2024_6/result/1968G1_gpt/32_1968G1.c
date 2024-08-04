use std::io;

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>()
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
        return r * r % MOD * a % MOD;
    }
    return r * r % MOD;
}

fn check(k: usize, n: usize, s: &Vec<char>, hash: &Vec<i64>, base_pow: &Vec<i64>) -> usize {
    let mut vo = 0;
    for i in 1..=n {
        if i + k - 1 <= n {
            if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
                vo += 1;
            }
        }
    }
    vo
}

fn main() {
    let t: usize = input_parse().unwrap();

    for _ in 0..t {
        let input_line: Vec<usize> = input_parse_as_vec().unwrap();
        let n = input_line[0];
        let l = input_line[1];
        let r = input_line[2];
        let s: String = input_parse().unwrap();

        let mut hash = vec![0; N + 2];
        let mut base_pow = vec![0; N + 1];
        let mut bp = 1;
        
        base_pow[0] = 1;
        for i in 1..=n {
            bp = bp * rand2(200, MOD as u32 - 1) as i64 % MOD;
            base_pow[i] = bp * rand2(200, MOD as u32 - 1) as i64 % MOD;
            hash[i] = bp * s.chars().nth(i-1).unwrap() as i64 % MOD;
            hash[i] = (hash[i] + hash[i - 1]) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &s.chars().collect(), &hash, &base_pow)) >= l as usize {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{}", lower);

        for i in l..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(mid, n, &s.chars().collect(), &hash, &base_pow)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!("");
    }
}

fn input_parse<T : std::str::FromStr>() -> Result<T, <T as std::str::FromStr>::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>()
}

fn input_parse_as_vec<T : std::str::FromStr>() -> Result<Vec<T>, <T as std::str::FromStr>::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let vec = input.split_whitespace()
                    .map(|x| x.parse::<T>().unwrap())
                    .collect();
    Ok(vec)
}