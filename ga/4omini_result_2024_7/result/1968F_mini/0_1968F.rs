use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

static N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Random {
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH).unwrap();
        let z = (duration.as_secs() ^ duration.subsec_micros() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], random: &mut Random, l: usize, r: usize) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (random.rand() as usize % (r - l))];

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
            } else {
                ii[j] as i32 - ii[i] as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                l += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }

        sort(ii, aa, random, l, i);
        l = k;
    }
}

fn search(a: u32, aa: &[u32], ii: &[usize], n: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < ii[lower as usize]) {
            lower = h;
        } else {
            upper = h;
        }
    }

    lower
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa: Vec<u32> = vec![0; n + 1];
        for i in 1..=n {
            line.clear();
            reader.read_line(&mut line).unwrap();
            let val: u32 = line.trim().parse().unwrap();
            aa[i] = aa[i - 1] ^ val;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        let mut random = Random::new();
        sort(&mut ii, &aa, &mut random, 0, n + 1);

        for _ in 0..q {
            line.clear();
            reader.read_line(&mut line).unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse::<usize>().unwrap();

            let result = if aa[l] == aa[r] || ii[(search(aa[r], &aa, &ii, n) + 1) as usize] < ii[search(aa[l], &aa, &ii, n) as usize] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}