use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let z = (start.as_secs() ^ start.subsec_micros() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, random: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (random.rand() % (r - l) as u32) as usize];

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
            } else {
                ii[j] as i32 - ii[i_] as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, aa, l, i, random);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, n: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < n as usize) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    let mut input = || lines.next().unwrap().unwrap();
    
    let t: usize = input().parse().unwrap();
    
    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = input();
            let mut iter = line.split_whitespace();
            (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let value: u32 = input().parse().unwrap();
            aa[i] = aa[i - 1] ^ value;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        let mut random = Random::new();
        sort(&mut ii, &aa, 0, n + 1, &mut random);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = input();
                let mut iter = line.split_whitespace();
                (iter.next().unwrap().parse::<usize>().unwrap() - 1, iter.next().unwrap().parse::<usize>().unwrap())
            };
            
            let result = if aa[l] == aa[r] || ii[(search(&aa, &ii, aa[r], n) + 1) as usize] < ii[search(&aa, &ii, aa[l], n) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}